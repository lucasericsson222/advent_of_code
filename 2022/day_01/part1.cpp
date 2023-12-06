
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <vector>
int main(int argc, char **argv) {
    if (argc != 2) {
        throw std::runtime_error("not enough arguments to executable");
    }
    std::ifstream file(argv[1]);
    
    std::vector<int> values;
    if (file.good()) {
        int i = 0;
        while (!file.eof()) {
            std::string line;
            std::getline(file, line);
            if (line != "") {
                std::stringstream ss(line);
                int number;
                ss >> number;
                if (values.size() < i + 1) {
                    values.push_back(number);
                } else {
                    values[i] += number;
                }
            } else {
                i += 1;
            }
        }
    } else {
        throw std::runtime_error("file could not be opened");
    }

    std::cout << *std::max_element(values.begin(), values.end()) << std::endl;
}
