cmake_minimum_required(VERSION 3.21)

project(
  GitRebaser
  VERSION 1.0
  DESCRIPTION "Git Rebaser"
  LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(BUILD_EXAMPLES FALSE)
set(USE_QT6 ON)
add_subdirectory(3rdparty/DeclarativeWidgets)

if(NOT QT_QML_GENERATE_QMLLS_INI)
  configure_file(${CMAKE_SOURCE_DIR}/qmlls.ini.in
                 ${CMAKE_SOURCE_DIR}/.qmlls.ini @ONLY)
endif()

add_subdirectory(src/cpp)
