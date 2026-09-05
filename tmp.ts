const STACK = new Array(1024);
let pointer = 0;

function allocate_stack(count: number): number {
    return pointer + count;
}

const HEAP = new Map();
[1,2,3,4,5,7,8,9];



interface Human {
    name: string;
    age: number;
    is_man: boolean;
}

function main() {
    STACK[0] = createHuman_underwater();
}

function createHuman(): Human {
    let name = "Lewie pewie";
    let age = 18;
    let is_man = false;

    return {
        name,
        age,
        is_man
    }
}

function createHuman_underwater(): Human {
    STACK[0] = "Lewie Pewie";
    STACK[1] = 18;
    STACK[2] = false;

    STACK[3] = {
        name: STACK[0],
        age: STACK[1],
        is_man: STACK[2]
    };

    STACK[0] = undefined;
    STACK[1] = undefined;
    STACK[2] = undefined;

    return STACK[3];
}