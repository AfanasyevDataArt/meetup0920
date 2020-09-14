#include <iostream>
#include <memory>

using namespace std;

void great(unique_ptr<string> name){
    cout<<"Hello, "<< *name <<"!"<<endl;
}

void great_ref(unique_ptr<string>& name){
    cout<<"Hello, "<< *name <<"!"<<endl;
}

int main() {
    auto alice = make_unique<string>("Alice");

    great_ref(alice);                   //Borrow
    great(make_unique<string>(*alice));    //Copy
    great(move(alice));                    //Move

    great_ref(alice);                   //Use after move.
}
