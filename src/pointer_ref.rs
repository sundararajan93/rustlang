// Reference Pointer - Point to a resource in memory

pub fn run() {
    //primitive array

    let array1 = [1, 2, 3, 4, 5];
    let array2 = array1;

    println!("Arrays - {:?}", (array1, array2));

    // non primitive array (vector) - If you assign another variable to a piece of data,
    // the first variable will no longer hold that value. 
    // You will need to use reference (&) to point the resource

    let vector1 = vec![1,2,3,4,5];
    let vector2 = &vector1;

    println!("Vectors - {:?}", (&vector1, vector2));

}
