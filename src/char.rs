// each character has iname, char stats??, matchups = vec<float>

// will have to normalize the typical -2, ..., +2 to [0, 1] -> sigmoid?

// get inspired from todo example project? chars 
struct Char {   
    id: i32,
    name: String,
    matchups: Vec<f32>
}

