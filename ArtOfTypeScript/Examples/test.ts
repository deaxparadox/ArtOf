interface Birds {
    fly(): void;
    layEggs(): void;
}

class Bird implements  Birds {
    fly() {
        console.log("flying...")
    }
    layEggs() {
        console.log("laying eggs...")
    }
}

let pet = new Bird();


// instanceof
if (pet instanceof Bird) {
    pet.fly();
} else {
    console.log("pet is not a bird");
}