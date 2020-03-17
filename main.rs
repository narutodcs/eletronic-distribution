#[derive(Debug)]
struct Atomic<'atom> {
    layers: [i32; 18],
    levels: [&'atom str; 18],
    support: [i32; 18]
}

fn main() {
    let mut numatom: usize = 22;
    let mut numhalf = 0;
    let mut numlevels = 0;

    let mut dist = Vec::new();
    let atom = Atomic {
        layers: [1, 2, 2, 3, 3, 4, 3, 4, 5, 4, 5, 6, 4, 5, 6, 7, 5, 6],
        levels: ["s", "s", "p", "s", "p", "s", "d", "p", "s", "d", "p", "s", "f", "d", "p", "s", "f", "d"],
        support: [2, 2, 6, 2, 6, 2, 10, 6, 2, 10, 6, 2, 14, 10, 6, 2, 14, 10]
    };
    
    for level in 0..18 {
        if (numatom as i32 - atom.support[level]) < 0 {
            numhalf = numatom;
        }
        if (numatom as i32 - atom.support[level]) >= 0 {
            numlevels += 1;
            numatom -= atom.support[level] as usize;
        } else {
            if numatom > 0 {
                numlevels += 1;
            }
            break;
        }
    }
    
    if numhalf == 0 {
        dist.push(atom.layers[0].to_string() + atom.levels[0] + &atom.support[0].to_string());
    } else {
        for level in 0..numlevels {
            if level == numlevels - 1 {
                dist.push(atom.layers[level].to_string() + atom.levels[level] + &numhalf.to_string());
            } else {
                dist.push(atom.layers[level].to_string() + atom.levels[level] + &atom.support[level].to_string());
            }
        }
    }
}
