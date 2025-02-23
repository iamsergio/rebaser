find_package(Qt6 REQUIRED COMPONENTS Gui Qml)
add_library(JsonModel SHARED jsonmodel.cpp)
target_link_libraries(JsonModel PRIVATE Qt6::Gui Qt6::Qml)

qt_add_qml_module(
  JsonModel
  URI
  JsonModel
  VERSION
  1.0
  SOURCES
  jsonmodel.cpp
  jsonmodel.h
  OUTPUT_DIRECTORY
  ${CMAKE_BINARY_DIR}/qml/JsonModel)
