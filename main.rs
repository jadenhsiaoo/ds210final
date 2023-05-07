mod functions; 

//main code that inputs the facebook and github's datasets
fn main() {
    let graph = functions::read_data("facebook_combined.txt");
    let average_distance = functions::calculate_average_shortest_path_length(&graph, 1000);
    println!("Average shortest path length for Facebook (approximated): {}", average_distance);
    let graph1 = functions::read_data("github.txt");
    let average_distance1 =functions::calculate_average_shortest_path_length(&graph1, 1000);
    println!("Average shortest path length for Github (approximated): {}", average_distance1);

    if average_distance > average_distance1 {
        println!("Github has the shortest path compared to Facebook");
    }else{
        println!("Facebook has the shortest path compared to Github");
    }

    }
