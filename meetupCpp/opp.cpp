#include <iostream>
#include <vector>

class BaseUnit{
    float hp = 100.0;
    float pos_x = 0.0;
    float pos_y = 0.0;
public:
    std::pair<float, float> get_pos() const {return std::make_pair(pos_x,pos_y);}
    void moveto(float x, float y){pos_x = x; pos_y = y;}
    void print() const {std::cout << "hp: " << hp<< "; pos: "<< pos_x << " , " << pos_y<<std::endl; }

    virtual void do_job()=0;
};

class Worker: public BaseUnit{
public:
    float stamina = 100.0;
    void do_job() override {
        std::cout << "Get "<< stamina << " resources" << std::endl;
    }
};

class Warrior: public BaseUnit{
public:
    float demage = 10.0;
    void do_job() override {
        std::cout << "Strike power: " << demage << std::endl;
    }
};

int main() {
   Worker worker;
   Warrior warrior;

   std::vector<BaseUnit*> units = {&worker, &warrior};

   for(auto unit: units){
       auto [x, y] = unit->get_pos();
       unit->moveto(x+1.0, y+1.0);
       unit->print();
       unit->do_job();
   }

   return 0;
}
