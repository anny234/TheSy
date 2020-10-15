from .import_lemmas import TheoryDocument



def find_in_path(filename: str, path: [str]):
    import os.path
    for d in path:
        full = os.path.join(d, filename)
        if os.path.exists(full): return full
    return None


def main():
    BENCHMARK_DIRS = ['frontend/benchmarks/isaplanner/via_hipster']
    COGNATE_SMT_DIRS = ['frontend/benchmarks/isaplanner/smt2']
    TARGET_DIRS = [] #'frontend/benchmarks/isaplanner']

    import os

    for td in TARGET_DIRS:
        try: os.makedirs(td)
        except FileExistsError: pass

    stats = {'theories': 0, 'lemmas': 0, 'mismatch': []}

    for d in BENCHMARK_DIRS:
        for fn in os.listdir(d):
            if fn.endswith('.thy'):
                print('--  %s  --' % fn)
                infile = open(os.path.join(d, fn))

                doc = TheoryDocument(infile)

                print(doc.datatypes, doc.ctors, doc.funcs)

                smtfn = find_in_path(fn.lower().replace('.thy', '.smt20.smt2'), COGNATE_SMT_DIRS)
                if smtfn:
                    with open(smtfn) as smtfile:
                        cognate_aliases = get_func_aliases(fn, doc, smtfile, stats)
                else:
                    cognate_aliases = None

                lemfn = os.path.join(d, fn + '.log')
                if os.path.exists(lemfn):
                    lemfile = open(lemfn)

                    doc = TheoryDocument(lemfile).merge(doc)
                    if cognate_aliases: doc.aliases = cognate_aliases
                    with open(os.path.join(d, fn.replace('.thy', '.goals.th')), 'w') as outf:
                        for t, lem in doc.lemmas:
                            goal = doc.export_lemma(lem, as_goal=True)
                            print(f" - {goal}")
                            print(goal, file=outf)
                            #print(f" - {t} {lem}")
                            #print(f"   {doc.find_vars(lem[0])} {doc.export_lemma(lem)}")
                    
                    if doc.lemmas:
                        stats['theories'] += 1
                        stats['lemmas'] += len(doc.lemmas)

    print(f"{stats['lemmas']} lemmas in {stats['theories']} theories")

    for (fn, th, smt) in stats['mismatch']:
        print(f"{fn}:  ", th, ' %% ', smt)


def grab_smt_declares(infile):
    import re
    decl = re.compile(r'\(declare-fun (.*?) ')
    for line in infile:
        mo = decl.match(line)
        if mo: yield mo.group(1)

def get_func_aliases(name, doc, infile, stats):
    cognate_funcs = [f for f in grab_smt_declares(infile)
                        if not f.startswith('apply')]
    print('%%', cognate_funcs)
    common = set(doc.funcs) & set(cognate_funcs)
    th, smt = [[f for f in l if f not in common]
                for l in [doc.funcs, cognate_funcs]]
    if len(th) != len(smt):
        stats['mismatch'].append((name, th, smt))

    return dict(zip(th, smt))



main()