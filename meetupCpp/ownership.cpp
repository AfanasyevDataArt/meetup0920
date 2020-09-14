#include <iostream>

using namespace std;

void great(string name){
    cout<<"Hello, "<<name <<"!"<<endl;
}

void great_ref(string& name){
    cout<<"Hello, "<<name <<"!"<<endl;
}

int main() {
    string alice = "Alice";

    great_ref(alice);   //Borrow
    great(alice);       //Copy
    great(move(alice)); //Move

    great(alice);       //Use after move. Why not?
}
