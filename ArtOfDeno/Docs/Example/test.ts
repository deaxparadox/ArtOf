interface Information {
    name(): string;
    age(): number;
    gender(): string;
    all(): string;
}


class Person implements Information {
    name(): string {
        return "Nitish";
    }
    age(): number {
        return 24;
    }
    gender(): string {
        return "M";
    }
    all(): string {
        return `${this.name()} ${this.age()} ${this.gender()}`;
    }
}

let p: Person = new Person();
console.log(p.all());