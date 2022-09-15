# Trigger the load of the Crypteia python package,
# that is going to apply wrapt to the os.environ object,
# so that we can customize the lookup of the environment
# variables at runtime. We cannot do this in LD_PRELOAD,
# because CPython uses LibC to look up only the PYTHON*
# environment variables, parsing directly the memory
# resolved by the linker to the __environ symbol as a
# Unix dictionary.

# IMPORTANT: This file must be valid Python 2.7+

from os.path import dirname
from sys import path, version, version_info

def import_crypteia():
    current_site = dirname(__file__)

    # We cannot use `sys.version_info.major` and other named attribute
    # got introduced only in Python 3.1
    ## TODO ADJUST FOR SUPPORT MATRIX
    if version_info[0] != 3 or version_info[1] < 7:
        path.remove(current_site)
        print("#CRYPTEIA# - Cannot import 'crypteia' due unsupported runtime version: {}".format(version))
        return

    # Reorder sys.path to put this site last and evaluate potential conflicts
    path.remove(current_site)
    path.append(current_site)

    # Importing pkg_resources must occur after we manipulate sys.path,
    # as the data about distributions is loaded on import
    import pkg_resources

    def _check_dependency_version_conflicts(package_name):
        distro = pkg_resources.get_distribution(package_name)

        version_conflicts = {}
        for requirement in distro.requires():
            try:
                pkg_resources.working_set.find(requirement)
            except pkg_resources.VersionConflict as e:
                version_conflicts[requirement.key] = {
                    'version_required': e.req.specs,
                    'version_found': e.dist.version,
                }

            version_conflicts.update(
                _check_dependency_version_conflicts(requirement.key)
            )

        return version_conflicts

    version_conflicts = _check_dependency_version_conflicts("crypteia")

    if not version_conflicts:
        import crypteia
    else:
        # Remove this site for good, we do not want to trigger issues
        path.remove(current_site)
        # Reload `pkg_resources` as otherwise it will recall distributions in this site
        from importlib import reload

        reload(pkg_resources)
        print("#CRYPTEIA# - Cannot import 'crypteia' due to dependency conflicts: {}".format(version_conflicts))

import_crypteia()
