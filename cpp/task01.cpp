#include "task01.h"
#include "util.h"
#include <iostream>
#include <ranges>

void task01() {
    auto text = get_task(1);
    auto chunks = std::ranges::views::split(text, "\r\n\r\n");
}