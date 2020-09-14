
#include <vector>
#include <string>
#include <fstream>
#include <iostream>
#include <chrono>
#include <algorithm>

using namespace std;

vector<string> lines_from_file(string filename){
    vector<string> lines;

    ifstream file(filename.c_str());
    string line;
    while(getline(file, line)){
        lines.push_back(line);
    }

    return std::move(lines);
}


int main(){
    auto lines = lines_from_file("/home/alexey/tmp/data.bin");

    cout<<"Read: "<< lines.size() <<" lines"<<endl;

    auto begin = chrono::steady_clock::now();
    sort(lines.begin(), lines.end());
    auto end = chrono::steady_clock::now();

    auto elapsed_ms = chrono::duration_cast<chrono::milliseconds>(end - begin);
    cout << "The time: " << elapsed_ms.count() << " ms\n";

    return 0;
}