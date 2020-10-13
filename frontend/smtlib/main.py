from .import_smtlib import SmtLibDocument
from .case_splits import ExtractCaseSplits


def main():
    # BENCHMARK_DIRS = ['benchmarks/cvc4-conj/original/benchmarks-dt/leon']
    BENCHMARK_DIRS = ['frontend/benchmarks/cvc4-conj/original/benchmarks-dt/isaplanner']
    TARGET_DIRS = ['frontend/benchmarks/isaplanner_translated']

    import os

    for td in TARGET_DIRS:
        try: os.makedirs(td)
        except FileExistsError: pass

    for (d, target_dir) in zip(BENCHMARK_DIRS, TARGET_DIRS):
        for fn in os.listdir(d):
            print('--  %s --' % fn)
            infile = open(os.path.join(d, fn))

            doc = SmtLibDocument(infile)
            with open(os.path.join(target_dir, fn + '.th'), 'w') as outf:
                for el in doc:
                    print(el)
                    print(el, file=outf)

                # for srule in ExtractCaseSplits(doc).guess_rules():
                #     print(srule)
                #     print(srule, file=outf)

            print(';', set(doc.iter_used_symbols()))
            print(';', set(doc.iter_used_types()))


if __name__ == '__main__':
    main()

