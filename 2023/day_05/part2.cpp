#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class range {
public:
  u_int32_t start, end;
  range(u_int32_t start, u_int32_t end) : start(start), end(end) {}

  range interesection(range &other) {
    auto end = std::min(other.end, this->end);
    auto start = std::max(other.start, this->start);

    if (end < start) {
      return empty();
    }

    return range(start, end);
  }
  std::vector<range> sub(range other) {

    std::vector<range> result;
    if (this->interesection(other) == empty()) {
      result.push_back(*this);
      return result;
    }
    if (other == empty()) {
      result.push_back(*this);
      return result;
    }
    if (other.end >= this->end && other.start >= this->start) {
      result.push_back(range(this->start, other.start - 1));
    }
    if (other.end <= this->end && other.start <= this->start) {
      result.push_back(range(other.end + 1, this->end));
    }
    if (other.end < this->end && other.start > this->start) {
      result.push_back(range(other.end + 1, this->end));
      result.push_back(range(this->start, other.start - 1));
    }
    std::vector<range> output;
    for (auto &r : result) {
      if (r.end >= r.start) {
        output.push_back(r);
      }
    }
    return output;
  }
  static range empty() { return range(0, 0); }
  range operator=(const range &other) {
    this->end = other.end;
    this->start = other.start;
    return *this;
  }
  bool operator==(const range &other) {
    if (other.start == this->start && other.end == this->end) {
      return true;
    }
    return false;
  }
  range shift(u_int32_t new_low) const {
    return range(new_low, this->end + new_low - this->start);
  }
};
class range_map {
private:
  std::vector<std::pair<range, u_int32_t>> ranges_to_dest;

public:
  void insert(range input_range, u_int32_t destination) {
    ranges_to_dest.push_back(
        std::pair<range, u_int32_t>(input_range, destination));
  }

  std::vector<range> find(std::vector<range> ins) {
    std::vector<range> out_ranges;
    std::vector<range> intersections;
    for (auto &in : ins) {
      std::vector<range> intermediate;
      for (auto &item : ranges_to_dest) {
        auto my_range = item.first;
        auto dest = item.second;
        auto intersection = my_range.interesection(in);
        if (intersection == range::empty()) {
          continue;
        }
        intersections.push_back(intersection);
        auto out =
            intersection.shift(dest + intersection.start - my_range.start);
        intermediate.push_back(out);
      }
      for (auto &item : intermediate) {
        out_ranges.push_back(item);
      }
    }
    for (auto &in : ins) {
      std::vector<range> our_range;
      our_range.push_back(in);
      for (auto &intersection : intersections) {
        std::vector<range> next_iter_range;
        for (auto &range : our_range) {
          for (auto &sub_range : range.sub(intersection)) {
            next_iter_range.push_back(sub_range);
          }
        }
        our_range = next_iter_range;
      }

      for (auto &range : our_range) {
        if (!(range == range::empty())) {
          out_ranges.push_back(range);
        }
      }
    }
    return out_ranges;
  }
};

range_map parse_map(std::string start_line, std::vector<std::string> &lines);
std::vector<range> parse_seed_numbers(std::string line);

int main() {
  std::ifstream file("input1.txt");

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
  for (auto seed : seeds) {
    std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
  }
  std::cout << "---------" << std::endl;
  auto seeds_to_soil = parse_map("seed-to-soil map:", lines);
  auto soil_to_fertilizer = parse_map("soil-to-fertilizer map:", lines);
  auto fertilizer_to_water = parse_map("fertilizer-to-water map:", lines);
  auto water_to_light = parse_map("water-to-light map:", lines);
  auto light_to_temperature = parse_map("light-to-temperature map:", lines);
  auto temperature_to_humidity =
      parse_map("temperature-to-humidity map:", lines);
  auto humidity_to_location = parse_map("humidity-to-location map:", lines);

  auto soil = seeds_to_soil.find(seeds);
  for (auto seed : soil) {
    std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
  }
  std::cout << "---------" << std::endl;
  auto fertilizer = soil_to_fertilizer.find(soil);
  for (auto seed : fertilizer) {
    std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
  }
  std::cout << "---------" << std::endl;
  auto water = fertilizer_to_water.find(fertilizer);
  for (auto seed : water) {
    std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
  }
  std::cout << "---------" << std::endl;
  auto light = water_to_light.find(water);
    for (auto seed : light) {
        std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
      }
      std::cout << "---------" << std::endl;
  auto temp = light_to_temperature.find(light);
    for (auto seed : temp) {
        std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
      }
      std::cout << "---------" << std::endl;
  auto humidity = temperature_to_humidity.find(temp);
    for (auto seed : humidity) {
        std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
      }
      std::cout << "---------" << std::endl;
  auto locations = humidity_to_location.find(humidity);
    for (auto seed : locations) {
        std::cout << "start: " << seed.start << ", end: " << seed.end << std::endl;
      }
      std::cout << "---------" << std::endl;

  u_int32_t min = locations[0].start;
  for (auto location : locations) {
    if (location.start < min) {
      min = location.start;
    }
  }

  std::cout << min << std::endl;
}

std::vector<range> parse_seed_numbers(std::string line) {
  std::vector<range> seeds;

  line.erase(0, 7); // erase "seeds"
  std::stringstream ss(line);

  u_int32_t range_start;
  u_int32_t range_length;
  while (!ss.eof()) {
    ss >> range_start;
    ss >> range_length;
    range new_range = {range_start, range_start + range_length - 1};
    seeds.push_back(new_range);
  }

  return seeds;
}

range_map parse_map(std::string start_line, std::vector<std::string> &lines) {
  range_map map;
  bool is_part_of_map = false;
  for (auto &line : lines) {
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

    range new_range = {source_start, source_start + range_length - 1};
    map.insert(new_range, destination_start);
  }
  return map;
}
