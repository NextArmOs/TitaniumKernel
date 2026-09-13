#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <chrono>
#include <thread>
#include <sstream>
#include <map>
#include <cstdlib>

#define RESET       "\033[0m"
#define TITANIUM_BG "\033[48;5;234m"
#define COPPER_TEXT "\033[38;5;173m"
#define GOLD_TEXT   "\033[38;5;220m"
#define GREEN_BAR   "\033[38;5;46m"
#define WHITE_TEXT  "\033[37m"

struct NetBytes {
    unsigned long long rx;
    unsigned long long tx;
};

std::map<std::string, NetBytes> get_network_bytes() {
    std::map<std::string, NetBytes> stats;
    std::ifstream file("/proc/net/dev");
    std::string line;
    if (!file.is_open()) return stats;
    
    std::getline(file, line);
    std::getline(file, line);

    while (std::getline(file, line)) {
        std::stringstream ss(line);
        std::string iface;
        ss >> iface;
        if (iface.empty()) continue;
        if (iface.back() == ':') iface.pop_back();
        if (iface == "lo") continue;

        unsigned long long rx = 0, tx = 0, dummy = 0;
        ss >> rx;
        for (int i = 0; i < 7; ++i) ss >> dummy;
        ss >> tx;

        stats[iface] = {rx, tx};
    }
    return stats;
}

int main() {
    auto prev_stats = get_network_bytes();
    
    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
        auto curr_stats = get_network_bytes();
        
        std::system("clear");
        std::cout << TITANIUM_BG;
        
        std::cout << WHITE_TEXT << R"(
    _______________________
    /   _________________   \
    |  |     _______     |  |     RJ45 ETHERNET
    |  | ___|       |___ |  | 
    |  ||               ||  |
    |  ||_______________||  |
    |  |                 |  |
    |  |  |||||||||||||  |  |
    |  |  |||||||||||||  |  |
    |  |________________|  |
    \_______________________/
 
          )" << RESET << TITANIUM_BG << "\n";

        std::cout << COPPER_TEXT << "==================================================" << RESET << TITANIUM_BG << "\n";
        std::cout << GOLD_TEXT   << "           PlatinumLinkNETWORK v0.0.4             " << RESET << TITANIUM_BG << "\n";
        std::cout << COPPER_TEXT << "==================================================" << RESET << TITANIUM_BG << "\n";
        std::printf(WHITE_TEXT   " %-15s | %-14s | %-14s\n", "Interface", "Download (RX)", "Upload (TX)");
        std::cout << COPPER_TEXT << "--------------------------------------------------" << RESET << TITANIUM_BG << "\n";

        for (const auto& [iface, data] : curr_stats) {
            if (prev_stats.find(iface) == prev_stats.end()) continue;

            double rx_speed = (data.rx - prev_stats[iface].rx) / 1024.0;
            double tx_speed = (data.tx - prev_stats[iface].tx) / 1024.0;

            int bars = static_cast<int>(rx_speed / 100.0);
            if (bars > 15) bars = 15;
            
            std::string graph = "";
            for (int b = 0; b < bars; ++b) graph += "#";
            for (int b = bars; b < 15; ++b) graph += " ";

            std::printf(WHITE_TEXT " %-15s " COPPER_TEXT "|" WHITE_TEXT " %6.1f KB/s     " COPPER_TEXT "|" WHITE_TEXT " %6.1f KB/s\n", iface.c_str(), rx_speed, tx_speed);
            std::cout << "                 " << COPPER_TEXT << "| " << WHITE_TEXT << "Graph: [" << GREEN_BAR << graph << WHITE_TEXT << "]\n" << RESET << TITANIUM_BG;
            std::cout << COPPER_TEXT << "--------------------------------------------------" << RESET << TITANIUM_BG << "\n";
        }
        std::cout << GOLD_TEXT << " [ Press Ctrl+C to close PlatinumLink panel ] " << RESET << "\n";
        
        prev_stats = curr_stats;
    }
    return 0;
}
