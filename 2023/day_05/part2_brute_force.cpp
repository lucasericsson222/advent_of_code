#include<fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<u_int32_t> parse_seed_numbers(std::string line); 

class range {
public:
    u_int32_t destination_start, source_start, range_length;
    range(u_int32_t destination_start, u_int32_t source_start, u_int32_t range_length)
    : destination_start(destination_start)
    , source_start(source_start)
    , range_length(range_length)
    {}
};
class range_map {
private:
    std::vector<range> ranges;
public:
    void insert(range input_range) {
        ranges.push_back(input_range);
    }

    u_int32_t find(u_int32_t in) {
        for (auto& range : ranges) {
            
            int64_t dif = in - range.source_start;
            if (dif < range.range_length && dif >= 0) {
                return dif + range.destination_start;
            }
        }
        return in;
    }
};

range_map parse_map(std::string start_line, std::vector<std::string>& lines); 
int main() {
    std::ifstream file("input.txt"); 
    
    std::vector<std::string> lines;

    if (file.good()) {
        while (!file.eof()) {
            std::string line;
            std::getline(file, line);
            lines.push_back(line);
        }
    } else {
        throw std::runtime_error("file could not be opened");
    }
    
    auto seeds_to_soil = parse_map("seed-to-soil map:", lines);
    auto soil_to_fertilizer = parse_map("soil-to-fertilizer map:", lines);
    auto fertilizer_to_water = parse_map("fertilizer-to-water map:", lines);
    auto water_to_light = parse_map("water-to-light map:", lines);
    auto light_to_temperature = parse_map("light-to-temperature map:", lines);
    auto temperature_to_humidity = parse_map("temperature-to-humidity map:", lines);
    auto humidity_to_location = parse_map("humidity-to-location map:", lines);

    u_int32_t min = -1;
    lines[0].erase(0, 7); // erase "seeds"
    std::stringstream ss(lines[0]);
    u_int32_t length;
    u_int32_t start;
    while (!ss.eof()) {
        ss >> start;
        ss >> length; 
        std::cout << "(start, length)" << "(" << start << ", " << length << ")" << std::endl;
        for (u_int32_t seed = start; seed < start + length; seed += 1) {
            auto soil = seeds_to_soil.find(seed);
            auto fertilizer = soil_to_fertilizer.find(soil);
            auto water = fertilizer_to_water.find(fertilizer);
            auto light = water_to_light.find(water);
            auto temp = light_to_temperature.find(light);
            auto humidity = temperature_to_humidity.find(temp);
            auto location = humidity_to_location.find(humidity);
            if (location < min) {
                min = location;
            }
        }
    }

    std::cout << min << std::endl;
}

std::vector<u_int32_t> parse_seed_numbers(std::string line) {
    std::vector<u_int32_t> seeds;
    
    line.erase(0, 7); // erase "seeds"
    std::stringstream ss(line);

    u_int32_t number;
    while (!ss.eof()) {
        ss >> number;
        seeds.push_back(number);
    }

    return seeds;
}

range_map parse_map(std::string start_line, std::vector<std::string>& lines) {
    range_map map;
    bool is_part_of_map = false;
    for (auto& line : lines) {
        if (line.find(start_line) != std::string::npos) {
            is_part_of_map = true;
            continue;
        }
        if (!is_part_of_map) {
            continue;
        }
        std::stringstream ss(line);

        u_int32_t destination_start, source_start, range_length;

        if (!(ss >> destination_start)) {
            break;
        }
        ss >> source_start;
        ss >> range_length;

        range new_range = {destination_start, source_start, range_length};
        map.insert(new_range); 
    }
    return map;
}
