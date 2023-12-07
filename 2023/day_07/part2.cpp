#include <algorithm>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

u_int64_t hand_power(std::string& hand); 

u_int64_t score_and_hand_left_right_val(int start, std::string& hand); 
int left_card_value(std::string& hand, int index); 
bool is_five_of_a_kind(std::unordered_map<char, int>& hand_counts); 
bool is_four_of_a_kind(std::unordered_map<char, int>& hand_counts); 
bool is_full_house(std::unordered_map<char, int>& hand_counts); 
bool is_three_of_a_kind(std::unordered_map<char, int>& hand_counts); 
bool is_two_pair(std::unordered_map<char, int>& hand_counts); 
bool is_one_pair(std::unordered_map<char, int>& hand_counts); 

int main() {
    std::ifstream  file("input.txt");

    std::vector<int> bids;
    std::vector<std::string> hands;

    if (file.good()) {
        while (!file.eof()) {
            std::string line;
            std::getline(file, line);
            if (line.length() == 0) {
                break;
            }
            hands.push_back(line.substr(0, 5));
            std::cout << line << std::endl;
            bids.push_back(std::stoi(line.substr(6, line.length()-6)));
        }
    }

    std::vector<std::pair<u_int64_t, int>> hand_power_to_bid;
    for (int i = 0; i < hands.size(); i += 1) {
        hand_power_to_bid.push_back(std::pair(hand_power(hands[i]), bids[i]));
    }

    std::sort(hand_power_to_bid.begin(), hand_power_to_bid.end());
    int sum = 0;
    int position = 1;
    for (auto& [hand_power, bid] : hand_power_to_bid) {
        std::cout << "(bid, position) (" << bid << ", " << position << ")" << std::endl; 
        sum += bid * position; 
        position += 1;
    }

    std::cout << "Sum: " << sum << std::endl;
}

u_int64_t hand_power(std::string& hand) {
    std::unordered_map<char, int> max_val_counts;

    for (auto ch : hand) {
        auto search = max_val_counts.find(ch);
        if (search != max_val_counts.end()) {
            search->second += 1;
        } else {
            max_val_counts.insert(std::pair(ch, 1));
        }
    }

    char max_occur_char = ' ';
    int max_count = 0;
    for (auto& [ch, count] : max_val_counts) {
        if (ch == 'J') {
            continue;
        }
        if (count > max_count) {
            max_count = count;
            max_occur_char = ch;
        }
    }
    
    std::string hand_two = "";
    for (auto ch: hand) {
        if (ch == 'J') {
            hand_two.push_back(max_occur_char);
        } else {
            hand_two.push_back(ch);
        }
    }

    std::unordered_map<char, int> val_counts;
    for (auto ch : hand_two) {
        auto search = val_counts.find(ch);
        if (search != val_counts.end()) {
            search->second += 1;
        } else {
            val_counts.insert(std::pair(ch, 1));
        }
    }

    if (is_five_of_a_kind(val_counts)) {
        return score_and_hand_left_right_val(7, hand);
    }

    if (is_four_of_a_kind(val_counts)) {
        return score_and_hand_left_right_val(6, hand);
    }
    if (is_full_house(val_counts)) {
        return score_and_hand_left_right_val(5, hand);
    }
    if (is_three_of_a_kind(val_counts)) {
        return score_and_hand_left_right_val(4, hand);
    }
    if (is_two_pair(val_counts)) {
        return score_and_hand_left_right_val(3, hand);
    }
    if (is_one_pair(val_counts)) {
        return score_and_hand_left_right_val(2, hand);
    }
    return score_and_hand_left_right_val(1, hand);
    
}

u_int64_t score_and_hand_left_right_val(int start, std::string& hand) {
    int index = 0;
    u_int64_t new_start = start;
    for (auto& ch : hand) {
        new_start *= 100;
        new_start += left_card_value(hand, index);
        index += 1;
    }

    return new_start;
}

int left_card_value(std::string& hand, int index) {
    char left = hand[index];
    if (left == 'A') {
        return 14;
    }
    if (left == 'K') {
        return 13;
    }
    if (left == 'Q') {
        return 12;
    }
    if (left == 'J') {
        return 1;
    }
    if (left == 'T') {
        return 10;
    }

    return left - '0';
}

bool is_five_of_a_kind(std::unordered_map<char, int>& hand_counts) {
    for (auto& [ch, val] : hand_counts) {
        if (val == 5) {
            return true;
        } else {
            return false;
        }
    }
    return false;
}

bool is_four_of_a_kind(std::unordered_map<char, int>& hand_counts) {
    for (auto& [ch, val] : hand_counts) {
        if (val == 4) {
            return true;
        } 
    }
    return false;
}

bool is_full_house(std::unordered_map<char, int>& hand_counts) {
    char three_of_a_kind_char = ' ';
    char two_of_a_kind_char = ' ';
    for (auto& [ch, val] : hand_counts) {
        if (val == 3) {
            three_of_a_kind_char = ch;
        }
        if (val == 2) {
            two_of_a_kind_char = ch;
        }
    }
    if (three_of_a_kind_char != ' ') {
        if (two_of_a_kind_char != ' ') {
            if (three_of_a_kind_char != two_of_a_kind_char) {
                return true;
            }
        }
    }
    return false;
}

bool is_three_of_a_kind(std::unordered_map<char, int>& hand_counts) {
    for (auto& [ch, val] : hand_counts) {
        if (val == 3) {
            return true;
        }
    }
    return false;
}

bool is_two_pair(std::unordered_map<char, int>& hand_counts) {
    char first_pair = ' ';
    for (auto& [ch, val] : hand_counts) {
        if (val == 2) {
            if (first_pair == ' ') {
                first_pair = ch;
            } else {
                if (first_pair != ch) {
                    return true;
                }
            }
        }
    }
    return false;
}

bool is_one_pair(std::unordered_map<char, int>& hand_counts) {
    for (auto& [ch, val] : hand_counts) {
        if (val == 2) {
            return true;
        }
    }
    return false;
}
