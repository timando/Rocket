# Simply sets up a few useful variables.

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

function relative() {
  local full_path="${SCRIPT_DIR}/../${1}"

  if [ -d "${full_path}" ]; then
    # Try to use readlink as a fallback to readpath for cross-platform compat.
    if command -v realpath >/dev/null 2>&1; then
      realpath "${full_path}"
    elif ! (readlink -f 2>&1 | grep illegal > /dev/null); then
      readlink -f "${full_path}"
    else
      echo "Rocket's scripts require 'realpath' or 'readlink -f' support." >&2
      echo "Install realpath or GNU readlink via your package manager." >&2
      echo "Aborting." >&2
      exit 1
    fi
  else
    # when the directory doesn't exist, fallback to this.
    echo "${full_path}"
  fi
}

function future_date() {
  local days_in_future=`[[ -z "$1" ]] && echo "0" || echo "$1"`
  if date -v+1d +%Y-%m-%d > /dev/null 2>&1; then
    echo $(date -v+${days_in_future}d +%Y-%m-%d)
  elif date -d "+1 day" > /dev/null 2>&1; then
    echo $(date '+%Y-%m-%d' -d "+${days_in_future} days")
  else
    echo "Error: need a 'date' cmd that accepts -v (BSD) or -d (GNU)"
    exit 1
  fi
}

# Versioning information. These are toggled as versions change.
CURRENT_RELEASE=true
PRE_RELEASE=false

# A generated codename for this version. Use the git branch for pre-releases.
case $PRE_RELEASE in
  true)
    VERSION_CODENAME="$(git branch --show-current)"
    ROCKET_VERSION="${VERSION_CODENAME}-$(future_date)"
    ;;
  false)
    ROCKET_VERSION="0.4.5"
    VERSION_CODENAME="$(echo "v${ROCKET_VERSION}" | cut -d'.' -f1-2)"
    ;;
esac

# Root of workspace-like directories.
PROJECT_ROOT=$(relative "") || exit $?
CORE_ROOT=$(relative "core") || exit $?
CONTRIB_ROOT=$(relative "contrib") || exit $?
SITE_ROOT=$(relative "site") || exit $?

# Root of project-like directories.
CORE_LIB_ROOT=$(relative "core/lib") || exit $?
CORE_CODEGEN_ROOT=$(relative "core/codegen") || exit $?
CORE_HTTP_ROOT=$(relative "core/http") || exit $?
CONTRIB_LIB_ROOT=$(relative "contrib/lib") || exit $?
CONTRIB_CODEGEN_ROOT=$(relative "contrib/codegen") || exit $?

# Root of infrastructure directories.
EXAMPLES_DIR=$(relative "examples") || exit $?
DOC_DIR=$(relative "target/doc") || exit $?

ALL_PROJECT_DIRS=(
    "${CORE_HTTP_ROOT}"
    "${CORE_CODEGEN_ROOT}"
    "${CORE_LIB_ROOT}"
    "${CONTRIB_CODEGEN_ROOT}"
    "${CONTRIB_LIB_ROOT}"
)

if [ "${1}" = "-p" ]; then
  echo "ROCKET_VERSION: ${ROCKET_VERSION}"
  echo "CURRENT_RELEASE: ${CURRENT_RELEASE}"
  echo "PRE_RELEASE: ${PRE_RELEASE}"
  echo "VERSION_CODENAME: ${VERSION_CODENAME}"
  echo "SCRIPT_DIR: ${SCRIPT_DIR}"
  echo "PROJECT_ROOT: ${PROJECT_ROOT}"
  echo "CORE_ROOT: ${CORE_ROOT}"
  echo "CONTRIB_ROOT: ${CONTRIB_ROOT}"
  echo "SITE_ROOT: ${SITE_ROOT}"
  echo "CORE_LIB_ROOT: ${CORE_LIB_ROOT}"
  echo "CORE_CODEGEN_ROOT: ${CORE_CODEGEN_ROOT}"
  echo "CORE_HTTP_ROOT: ${CORE_HTTP_ROOT}"
  echo "CONTRIB_LIB_ROOT: ${CONTRIB_LIB_ROOT}"
  echo "CONTRIB_CODEGEN_ROOT: ${CONTRIB_CODEGEN_ROOT}"
  echo "EXAMPLES_DIR: ${EXAMPLES_DIR}"
  echo "DOC_DIR: ${DOC_DIR}"
  echo "ALL_PROJECT_DIRS: ${ALL_PROJECT_DIRS[*]}"
  echo "date(): $(future_date)"
fi
