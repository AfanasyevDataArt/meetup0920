
#include <vector>
#include <string>
#include <fstream>
#include <iostream>
#include <chrono>
#include <algorithm>
#include <cstring>

using namespace std;

static const std::size_t SIZE = 128;

class LineBuf{
public:
    LineBuf(string s = ""){
        std::memcpy(buf, s.c_str(), (s.size()<SIZE)?s.size():SIZE);
    }
    char buf[SIZE] = {0};

    bool operator < (LineBuf& rval){
        return std::memcmp(buf, rval.buf, SIZE) < 0;
    }
};

template<class T>
void lines_from_file(vector<T> &lines, string filename){

    ifstream file(filename.c_str());
    string line;
    while(getline(file, line)){
        for(size_t pos=0; pos<line.size(); pos+= SIZE){
            auto str = line.substr(pos, SIZE);
            lines.emplace_back(str);
        }

    }
}

template<class T>
void bench(vector<T> &lines){
    cout<<"Read: "<< lines.size() <<" lines"<<endl;

    auto begin = chrono::steady_clock::now();
    sort(lines.begin(), lines.end());
    auto end = chrono::steady_clock::now();

    auto elapsed_ms = chrono::duration_cast<chrono::milliseconds>(end - begin);
    cout << "The time: " << elapsed_ms.count() << " ms\n";
}

int main(){
    cout<<"Test vector<strings>:"<<endl;
    vector<string> lines_str;
    lines_from_file<string>(lines_str, "/home/alexey/tmp/data.bin");
    bench<string>(lines_str);

    cout<<"Test vector<char["<<SIZE<<"]>:"<<endl;
    vector<LineBuf> lines_custom;
    lines_from_file<LineBuf>(lines_custom, "/home/alexey/tmp/data.bin");
    bench<LineBuf>(lines_custom);

    return 0;
}
