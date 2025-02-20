cmake_minimum_required(VERSION 3.21)

project(GitRebaser
    VERSION 1.0
    DESCRIPTION "Git Rebaser"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(3rdparty/DeclarativeWidgets)

configure_file(
    ${CMAKE_SOURCE_DIR}/qmlls.ini.in
    ${CMAKE_SOURCE_DIR}/.qmlls.ini
    @ONLY
)
