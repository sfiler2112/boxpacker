use std::io;

pub trait Volume {
  fn get_volume(&self) -> f64;
}

#[derive(Copy, Clone)]
pub struct RectangularPrism {
  pub height: f64,
  pub width: f64,
  pub depth: f64,
}


impl Volume for RectangularPrism {
  fn get_volume(&self) -> f64 {
    self.height * self.width * self.depth
  }
}

pub trait Rotation {
  fn rotate(&self, orientation: &Orientation) -> RectangularPrism;
}

impl Rotation for RectangularPrism {
  fn rotate(&self, orientation: &Orientation) -> RectangularPrism {
    let mut rotated_height = self.height;
    let mut rotated_width = self.width;
    let mut rotated_depth = self.depth;

    if orientation.x_axis != 0 {
      let temp_height = rotated_height;
      rotated_height = rotated_depth;
      rotated_depth = temp_height;
    }
    if orientation.y_axis != 0 {
      (rotated_depth, rotated_width) = (rotated_width, rotated_depth);
    }
    if orientation.z_axis != 0 {
      (rotated_height, rotated_width) = (rotated_width, rotated_height);
    }


    RectangularPrism {
      height: rotated_height,
      width: rotated_width,
      depth: rotated_depth
    }
  }
}

/*
 * Orientations that we care about:
 *
 *   x | y | z
 *   ---------
 *   0 | 0 | 0  [Standard height, width, and depth]
 *   1 | 0 | 0  [Height and depth swapped]
 *   1 | 1 | 0  [Height and depth swapped, then width and depth swapped]
 *   1 | 0 | 1  [Height and depth swapped, then height and width swapped]
 *   0 | 1 | 0  [Width and depth swapped]
 *   0 | 0 | 1  [Height and width swapped]
 *
 * Heap's Algorithm may be the best way of going through each combo.
 * My problem is associating specific orientations with dimension values for the end-user
 * output.
 *
 */

#[derive(Copy, Clone)]
pub struct Orientation {
  x_axis: i32,
  y_axis: i32,
  z_axis: i32,
}

impl Default for Orientation {
  fn default() -> Self {
    Orientation {
      x_axis: 0,
      y_axis: 0,
      z_axis: 0,
    }
  }
}

impl Orientation {
  fn set_orientation(&mut self, x: i32, y: i32, z: i32) {
    self.x_axis = x;
    self.y_axis = y;
    self.z_axis = z;
  }

  fn get_orientation_tuple(&self) -> (i32, i32, i32) {
    (self.x_axis, self.y_axis, self.z_axis)
  }
}

pub struct Product {
  dimensions: RectangularPrism,
  orientation: Orientation
}

impl Product {
  fn get_rotated_dimensions(&self) -> RectangularPrism {
    self.dimensions.rotate(&self.orientation)
  }
}

pub struct Container {
  dimensions: RectangularPrism,
}

impl Container {
  fn get_product_quantity_per_layer(&self) -> i32 {
    0
  }

  fn get_volume(&self) -> f64 {
    self.dimensions.get_volume()
  }
}

pub struct BoxPacker {
  container: Container,
  product: Product,
}

fn orientation_generator((x, y, z): (i32, i32, i32)) -> Orientation {
  Orientation {
    x_axis: x,
    y_axis: y,
    z_axis: z,
  }
}

impl BoxPacker {
  fn get_optimal_orientation(&mut self) -> Orientation {
    /* 
     * Determine maximum amount of products that can be packed into the container for each orientation.
     * Return the orientation with the highest maximum
     */

    println!("#############################################################");
    println!("# The boxpacker is finding the optimal product orientation! #");
    println!("#############################################################");


    let mut optimal_orientation: Orientation = Default::default(); /* default orientation is 0,0,0 */
    println!("starting values for optimal_orientation: {} {} {}", optimal_orientation.x_axis, optimal_orientation.y_axis, optimal_orientation.z_axis);

    let mut max_products_packable = self.get_num_products_packable(self.product.dimensions);
    println!("Max products packable with starting orientation: {}", max_products_packable);
    
    let alt_orientations = vec![
      orientation_generator((1,0,0)),
      orientation_generator((1,1,0)),
      orientation_generator((1,0,1)),
      orientation_generator((0,0,1)),
      orientation_generator((0,1,0)),
    ];

    for altori in alt_orientations {
      /* set the product orientation to the current alternative */
      self.product.orientation = altori;

      /* get the dimensions for this orientation */
      let alt_dim = self.product.get_rotated_dimensions();

      /* get the number of products (columns x rows x layers) that fit in the container */
      /*
      let columns = (self.container.dimensions.width / alt_dim.width) as i32;
      println!("columns: {}", columns);
      let rows = (self.container.dimensions.depth / alt_dim.depth) as i32;
      println!("rows: {}", rows);
      let layers = (self.container.dimensions.height / alt_dim.height) as i32;
      println!("layers: {}", layers);
      let alt_products_packable = columns * rows * layers;
      */
      let alt_products_packable = self.get_num_products_packable(alt_dim);
      println!("number of products packable: {}", alt_products_packable);

      /* compare to the number of products for the current optimal orientation */
      if alt_products_packable > max_products_packable {
        max_products_packable = alt_products_packable;
        optimal_orientation = altori;
        println!("new opt ori found!");
      } else {
        println!("not good enough!! >:(");
      }
    }
    optimal_orientation
  }

  fn get_num_products_packable(&self, prod_dimensions: RectangularPrism) -> i32 {
    ((self.container.dimensions.width / prod_dimensions.width) as i32)
    * ((self.container.dimensions.depth / prod_dimensions.depth) as i32)
    * ((self.container.dimensions.height / prod_dimensions.height) as i32)
  }
}

fn user_create_rectangular_prism() -> RectangularPrism {
  println!("Collecting the dimensions for a rectangular prism!");
  println!("Please enter the value in inches for each of the following...");

  RectangularPrism {
    height: collect_user_dimension_value("Height"),
    width: collect_user_dimension_value("Width"),
    depth: collect_user_dimension_value("Depth"),
  }
}

fn collect_user_dimension_value(dim: &str) -> f64 {
  /* Display the name of dimension being collected for the user */
  println!("{dim}:");

  /* Collect the dimension value as a string */
  let mut dim_value_str = String::new();
  io::stdin().read_line(&mut dim_value_str).unwrap();
  
  /* Return the dimension value parsed into type f64 */
  dim_value_str.trim().parse().unwrap()
}

fn user_create_container() -> Container {
  println!("*************************************************");
  println!("* Create the container that we will be packing! *");
  println!("*************************************************");  
 
  let user_cont = Container {
    dimensions: user_create_rectangular_prism(),
  };

  println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

  user_cont
}

fn user_create_product() -> Product {
  println!("*******************************************");
  println!("* Create the product that will be packed! *");
  println!("*******************************************");

  let user_prod = Product {
    dimensions: user_create_rectangular_prism(),
    orientation: Default::default()
  };

  println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

  user_prod
}

#[derive(Debug, Copy, Clone)]
enum MainMenuOption {
  CreateBoxPacker,
  RunBoxPacker,
  Exit,
}

#[derive(Debug)]
struct MainMenuItem {
  option: MainMenuOption,
  text: String
}

fn display_main_menu_options(menu_options: &[MainMenuItem]) {
  println!("");

  /* Display the text for each menu option */
  let mut index = 1;
  for option in menu_options {
    println!("[{index}] {}", option.text);
    index += 1;
  };

}

fn get_user_selection() -> usize {
  let mut user_selection = String::new();
  println!("Please enter your selection:");
  io::stdin().read_line(&mut user_selection).unwrap();

  user_selection.trim().parse::<usize>().unwrap()
}

fn run_main_menu() {
  let options: [MainMenuItem; 3];
  options = [
    MainMenuItem {
      option: MainMenuOption::CreateBoxPacker,
      text: String::from("Create a new Box Packer!"), 
    },
    MainMenuItem {
      option: MainMenuOption::RunBoxPacker,
      text: String::from("Start using the Box Packer!"),
    },
    MainMenuItem {
      option: MainMenuOption::Exit,
      text: String::from("Exit the program!"),
    },
  ];
 
  /* 
   * Show the main menu options the user can choose from.
   * The options are numbered according to their order in "options"
   * options[0] is listed as 1, option[1] as 2, etc.
   */
  display_main_menu_options(&options);
  /* 
   * The user selection is returned as the number the options was listed as.
   * 1 must be subtracted from the user selection to get the index for the selected option.
   */
  let selected_option = options[get_user_selection() - 1].option;
  println!("{:?}", selected_option);
}

fn main() {
    run_main_menu();
    /* 
     * Get the container dimensions from the user
     */
    let container_box = user_create_container();

    /*
     * Get the product dimensions from the user
     */
    /*
    let mut height = String::new();
    println!("What is the product box height?");
    io::stdin().read_line(&mut height).unwrap();
    println!("Product box height: {}", height);
    println!("Lets hope container height is still fine: {}", container_box.dimensions.height);

    let mut product_box = Product {
      dimensions: RectangularPrism {
        height: 6.0,
        width: 4.0,
        depth: 1.0,
      },
      orientation: Orientation {
        x_axis: 0,
        y_axis: 0,
        z_axis: 0,
      },
    };
    */
    let product_box = user_create_product();

    let mut box_packer_boy = BoxPacker {
      container: container_box,
      product: product_box
    };
    let optori = box_packer_boy.get_optimal_orientation();
    println!("optori values: {} {} {}", optori.x_axis, optori.y_axis, optori.z_axis);
}
