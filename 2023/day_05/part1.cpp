#include<fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<int> parse_seed_numbers(std::string line); 
std::unordered_map<int, int> parse_map(std::string start_line, std::vector<std::string>& lines); 
int get_padded(int in, std::unordered_map<int, int>& map); 

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
    
    auto seeds = parse_seed_numbers(lines[0]);
    std::cout << "First seed: " << seeds[0] << std::endl;
    std::cout << "Parse seeds is okay" << std::endl;    

    auto seeds_to_soil = parse_map("seed-to-soil map:", lines);
    auto soil_to_fertilizer = parse_map("soil-to-fertilizer map:", lines);
    auto fertilizer_to_water = parse_map("fertilizer-to-water map:", lines);
    auto water_to_light = parse_map("water-to-light map:", lines);
    auto light_to_temperature = parse_map("light-to-temperature map:", lines);
    auto temperature_to_humidity = parse_map("temperature-to-humidity map:", lines);
    auto humidity_to_location = parse_map("humidity-to-location map:", lines);
    std::cout << "Parse map is okay" << std::endl;

    std::vector<int> locations;
    for (auto seed : seeds) {
        auto soil = get_padded(seed, seeds_to_soil);
        std::cout << "soil:" << soil << std::endl;
        auto fertilizer = get_padded(soil, soil_to_fertilizer);
        auto water = get_padded(fertilizer, fertilizer_to_water);
        auto light = get_padded(water, water_to_light);
        auto temp = get_padded(light, light_to_temperature);
        auto humidity = get_padded(temp, temperature_to_humidity);
        auto location = get_padded(humidity, humidity_to_location);
        locations.push_back(location);
    }
    std::cout << "Get locations okay" << std::endl;
    int min = locations[0];
    std::cout << "there is a location" << std::endl;

    for (auto location : locations) {
        if (location < min) {
            min = location;
        }
    }
    std::cout << "there is a min" << std::endl;

    std::cout << min << std::endl;
}

int get_padded(int in, std::unordered_map<int, int>& map) {
    auto search = map.find(in);
    if (search != map.end()) {
        return search->second;
    }
    return in;
}

std::vector<int> parse_seed_numbers(std::string line) {
    std::vector<int> seeds;
    
    line.erase(0, 7); // erase "seeds"
    std::stringstream ss(line);

    int number;
    while (!ss.eof()) {
        ss >> number;
        std::cout << "Seed number: " << number << std::endl;
        seeds.push_back(number);
    }

    return seeds;
}

std::unordered_map<int, int> parse_map(std::string start_line, std::vector<std::string>& lines) {
    std::unordered_map<int, int> map;
    bool is_part_of_map = false;
    for (auto& line : lines) {
        if (line.find(start_line) != std::string::npos) {
            is_part_of_map = true;
            continue;
        }
        std::stringstream ss(line);

        int destination_start, source_start, range_length;

        if (!(ss >> destination_start)) {
            break;
        }
        ss >> source_start;
        ss >> range_length;

        for (int i = 0; i < range_length; i++) {
            map.insert({source_start + i, destination_start + i}); 
        }
    }
    return map;
}
