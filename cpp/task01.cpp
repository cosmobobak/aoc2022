#include "task01.h"

#include <algorithm>
#include <charconv>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>
#include <numeric>

#include "util.h"

namespace rng = std::ranges;
namespace vws = std::views;
using std::string_view;
using std::vector;
using std::array;

void task01() {
    constexpr auto two_line_delim = string_view{"\r\n\r\n"};
    constexpr auto line_delim = string_view{"\r\n"};
    auto text = get_task(1);
    auto nums_range = text 
        | vws::split(two_line_delim) 
        | vws::transform([=](auto&& rng) {
            auto block = string_view(&*rng.begin(), rng::distance(rng));
            auto sum = 0ull;
            auto add_to_sum = [&sum](const auto val) { sum += val; };
            rng::for_each(block 
                | vws::split(line_delim) 
                | vws::transform([](auto&& rng) {
                    auto line = string_view(&*rng.begin(), rng::distance(rng));
                    auto num = 0ull;
                    std::from_chars(line.begin(), line.end(), num);
                    return num;
                }),
                add_to_sum);
            return sum;
            });
    
    auto nums = vector<unsigned long long>{};
    rng::copy(nums_range, std::back_inserter(nums));

    auto top3 = array{0ull, 0ull, 0ull};
    // select highest 3
    for (auto i = 0; i < 3; ++i) {
        auto max_loc = rng::max_element(nums);
        top3[i] = *max_loc;
        nums.erase(max_loc);
    }

    std::printf("Part 1: %llu\r\n", top3[0]);
    std::printf("Part 2: %llu\r\n", std::accumulate(top3.begin(), top3.end(), 0ull));
}