pub fn vectorDiffTypes(){
    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f32),
        Text(String)
    }

    let temp = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(1.23),
        SpreadsheetCell::Text(String::from("Shivam"))
    ];
    
    for ele in &temp {
        println!("{:?}",ele);
    }

    match &temp[1] {
        SpreadsheetCell::Int(val) => println!("INT : {val}"),
        SpreadsheetCell::Float(val) => println!("FLOAT : {val}"),
        SpreadsheetCell::Text(val) => println!("TEXT : {val}")
    }
}



pub fn vectorUse(){
    //Array : 
    let a = [1,2,3];


    //vector : 
    {
        //first way
        let mut b:Vec<i32> = Vec::new();
        b.push(1);
        b.push(2);
        b.push(3);


        //how to access elements from the vector
        {   
            //first way
            //it will give error when we access any element out of range
            println!("Element at index : {} is {}",0,&b[0]);
        }

        {
            //second way
            let element =  b.get(20);
            match element {
                Some(element) => 
                println!("Element at index : {} is {}",0,element),
                _ => println!("There is no such element.")
            }
        }

        //Iterating the vectors
        for (ind,ele) in b.iter().enumerate(){
            println!("element at index : {ind} is {ele}");
        }

        //we can also take mutable reference to change the values of vector
        for ele in &mut b{
            *ele += 1;
        }

        println!("New vector : {:?}",b);
    }

    
    {
        //second way
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6); will give error because we are using immutable reference
        // in the below line. 
        println!("The first element is: {:?}",first);
    }
    
}