
import os

def main():
    os.chdir('..')
    orig = set(fname for fname in os.listdir('orig') if fname.endswith('.rs') and fname[0].isdigit())
    links2orig = set(os.path.basename(os.readlink(fname))
                  for fname in os.listdir('.')
                  if fname.endswith('.rs') and os.path.islink(fname))
    for lost in orig-links2orig:
        src = os.path.join('orig', lost)
        link_name = ".".join(lost.split('.')[1:]).replace('-', '_')
        print(f"create link {link_name} -> {src}")
        os.symlink(src, link_name)


def rm_links():
    links = (fname for fname in os.listdir('.') if fname.endswith('.rs')
             and os.path.islink(fname))
    for fname in links:
        os.remove(fname)


if __name__ == '__main__':
    main()
    # rm_links()
