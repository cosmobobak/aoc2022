#include "util.h"
#include <string>
#include <fstream>
#include <sstream>

auto get_task(size_t task) -> std::string {
    // read file at path "tasks/task{task:02}.txt" into a string
    // task 1: "tasks/task01.txt"
    auto path = std::string { "tasks/task" };
    if (task < 10) {
        path += "0";
    }
    path += std::to_string(task) + ".txt";
    auto file = std::ifstream { path };
    auto ss = std::stringstream {};
    ss << file.rdbuf();
    return ss.str();
}

