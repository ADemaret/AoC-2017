///
/// Réécriture de grid (array 2D) en utilisant les vecteurs
/// pour ne pas devoir fixer de taille en dur
///
//use crate::utils::pause;
use std::{collections::VecDeque, fmt};

/// Au dessus de MAX_AFFICHAGE, print() ne fait rien
const MAX_AFFICHAGE: usize = 99;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D {
    pub max_l: usize,
    pub max_c: usize,
    pub grid: Vec<Vec<char>>,
}

impl Grid2D {
    pub fn new(input: &str) -> Grid2D {
        let mut grid = Vec::new();
        let max_l = input.lines().count(); // nombre de lignes
        let max_c = input.lines().next().unwrap().chars().count(); // taille de la première ligne

        input.lines().enumerate().for_each(|(l,line)| {
            if line.chars().count() != max_c {
                panic!("** ERREUR : Ce n'est pas une grille : la ligne {} n'a pas la même longueur que la ligne 1",l+1);
            } else {
                let mut vec_l = Vec::new();
                    line.chars().for_each(|ch| {vec_l.push(ch);});
                    grid.push(vec_l);
            }
            });
        Grid2D { max_l, max_c, grid }
    }

    pub fn new_by_sizes(width: usize, height: usize, default_char: char) -> Grid2D {
        let max_l = height;
        let max_c = width;
        Grid2D {
            max_l,
            max_c,
            grid: vec![vec![default_char; width]; height],
        }
    }

    ///
    /// Affiche la grille avec numéros de lignes et de colonnes
    ///
    pub fn print(&self) {
        if self.max_l > MAX_AFFICHAGE || self.max_c > MAX_AFFICHAGE {
            println!(
                "** WARNING : la grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        print!(" C-> ");
        for c in 0..self.max_c {
            print!(" {:#02}", c);
        }
        println!();

        // cadre supérieur
        print!("    ┌");
        for _ in 0..self.max_c {
            print!("───");
        }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l, line)| {
            print!("L{:#02} │", l);
            line.iter().for_each(|ch| {
                print!("  {}", ch);
            });
            println!(" │");
        });

        // bord inférieur
        print!("    └");
        for _ in 0..self.max_c {
            print!("───");
        }
        println!("─┘");
    }

    ///
    /// Affiche la grille avec numéros de lignes et de colonnes
    ///
    pub fn print_portion(&self, l_min: usize, c_min: usize, l_max: &mut usize, c_max: &mut usize) {
        *l_max = self.max_l.min(*l_max);
        *c_max = self.max_c.min(*c_max);

        if (*l_max - l_min) > MAX_AFFICHAGE || (*c_max - c_min) > MAX_AFFICHAGE {
            println!(
                "** WARNING : la grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        print!(" C-> ");
        for c in c_min..*c_max {
            print!(" {:#02}", c);
        }
        println!();

        // cadre supérieur
        print!("    ┌");
        for _ in c_min..*c_max {
            print!("───");
        }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l, line)| {
            if l >= l_min && l < *l_max {
                print!("L{:#02} │", l);
                line.iter().enumerate().for_each(|(c, ch)| {
                    if c >= c_min && c < *c_max {
                        print!("  {}", ch);
                    }
                });
                println!(" │");
            }
        });

        // bord inférieur
        print!("    └");
        for _ in c_min..*c_max {
            print!("───");
        }
        println!("─┘");
    }

    ///
    /// Affiche la grille avec numéros de lignes et de colonnes
    /// et les points d'un vecteur de points
    ///
    pub fn print_with_vec(&self, v: &[(usize, usize)], display_char: char) {
        if self.max_l > MAX_AFFICHAGE || self.max_c > MAX_AFFICHAGE {
            println!(
                "** WARNING : la grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        print!(" C-> ");
        for c in 0..self.max_c {
            print!(" {:#02}", c);
        }
        println!();

        // cadre supérieur
        print!("    ┌");
        for _ in 0..self.max_c {
            print!("───");
        }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l, line)| {
            print!("L{:#02} │", l);
            line.iter().enumerate().for_each(|(c, ch)| {
                if v.contains(&(l, c)) {
                    print!("  {}", display_char);
                } else {
                    print!("  {}", ch);
                }
            });
            println!(" │");
        });

        // bord inférieur
        print!("    └");
        for _ in 0..self.max_c {
            print!("───");
        }
        println!("─┘");
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans la grille
    ///
    pub fn count_occurences(&self, cc: char) -> usize {
        self.grid.iter().flatten().filter(|&c| *c == cc).count()
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans une ligne
    ///
    pub fn count_occurences_in_line(&self, cc: char, line: usize) -> usize {
        self.grid[line].iter().filter(|&c| *c == cc).count()
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans une colonne
    ///
    pub fn count_occurences_in_column(&self, cc: char, col: usize) -> usize {
        let mut count = 0;
        for l in 0..self.max_l {
            if self.grid[l][col] == cc {
                count += 1;
            }
        }
        count
    }

    ///
    /// Renvoie un vecteur des coordonnées d'un caractère
    ///
    pub fn get_vec_of_char_positions(&self, cc: char) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();

        self.grid.iter().enumerate().for_each(|(l, line)| {
            line.iter().enumerate().for_each(|(c, ch)| {
                if ch.eq(&cc) {
                    vec.push((l, c));
                }
            })
        });
        vec
    }

    ///
    /// Renvoie un vecteur des caractères des 8 cases adjacentes
    /// (gauche, droite, haut, bas et diagonales)
    ///
    pub fn get_adjacents(&self, l: usize, c: usize) -> Vec<(usize, usize, char)> {
        let mut ret = Vec::new();
        let adj: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dl, dc) in adj {
            if (dl < 0 && l == 0)
                || (dc < 0 && c == 0)
                || (dl > 0 && l + 1 == self.max_l)
                || (dc > 0 && c + 1 == self.max_c)
            {
            } else {
                let new_l = (dl + (l as i32)) as usize;
                let new_c = (dc + (c as i32)) as usize;
                ret.push((new_l, new_c, self.grid[new_l][new_c]));
            }
        }
        ret
    }

    ///
    /// rotate the grid 90° counter-clockwise
    ///
    #[allow(clippy::needless_range_loop)]
    pub fn rotate(self) -> Grid2D {
        let mut new_grid: Vec<Vec<char>> = (0..self.max_c).map(|_| Vec::new()).collect();
        for l in 0..self.max_l {
            for c in 0..self.max_c {
                new_grid[self.max_c - c - 1].push(self.grid[l][c]);
            }
        }
        Grid2D {
            max_l: self.max_c,
            max_c: self.max_l,
            grid: new_grid,
        }
    }

    pub fn set_rect(&mut self, a: usize, b: usize, ch: char) {
        for c in 0..a {
            for l in 0..b {
                self.grid[l][c] = ch;
            }
        }
    }

    ///
    /// shifts all of the pixels in column A down by B pixels
    ///
    pub fn shift_col_down(&mut self, col: usize, moves: usize) {
        let mut new_grid = self.grid.clone();
        for l in 0..self.max_l {
            if l + moves >= self.max_l {
                new_grid[l + moves - self.max_l][col] = self.grid[l][col];
            } else {
                new_grid[l + moves][col] = self.grid[l][col];
            }
        }
        self.grid = new_grid;
    }

    pub fn shift_row_right(&mut self, row: usize, moves: usize) {
        let mut new_grid = self.grid.clone();
        for c in 0..self.max_c {
            if c + moves >= self.max_c {
                new_grid[row][c + moves - self.max_c] = self.grid[row][c];
            } else {
                new_grid[row][c + moves] = self.grid[row][c];
            }
        }
        self.grid = new_grid;
    }

    /// attribue un même nombre (n° de région) aux cellules contenant '#'
    /// qui se touchent horizontalement ou verticalement    
    pub fn count_areas(&self) -> Option<usize> {
        // régions
        let mut grid_reg = vec![vec![RegCell::Empty; self.max_c]; self.max_l];
        for l in 0..self.max_l {
            for c in 0..self.max_c {
                if self.grid[l][c] == '1' {
                    grid_reg[l][c] = RegCell::Region(None);
                }
            }
        }

        // find next cell without a reg
        let mut reg_nbr = 0;
        for l in 0..self.max_l {
            for c in 0..self.max_c {
                if grid_reg[l][c] == RegCell::Region(None) {
                    grid_reg[l][c] = RegCell::Region(Some(reg_nbr));
                    set_neightbours(&mut grid_reg, l, c, self.max_l, self.max_c, reg_nbr);
                    reg_nbr += 1;
                }
            }
        }
        // Debug
        // for l in 0..8 {
        //     for c in 0..8 {
        //         print!("{}", grid_reg[l][c]);
        //     }
        //     println!();
        // }
        Some(reg_nbr)
    }
}

////////////////////////////////////////////////////////////////////////////
/// RegCell
#[derive(Debug, Clone, PartialEq)]
enum RegCell {
    Empty,
    Region(Option<usize>),
}
impl fmt::Display for RegCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegCell::Empty => write!(f, "."),
            RegCell::Region(None) => write!(f, "?"),
            RegCell::Region(Some(r)) => write!(f, "{}", r),
        }
    }
}
// end of RegCell
fn set_neightbours(
    grid_reg: &mut Vec<Vec<RegCell>>,
    l: usize,
    c: usize,
    max_l: usize,
    max_c: usize,
    reg_nbr: usize,
) {
    let mut queue = VecDeque::new();
    let mut dejavu = vec![(l, c)];

    let adj: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    queue.push_front((l, c));

    while let Some((l, c)) = queue.pop_front() {
        for (dl, dc) in adj {
            if (dl < 0 && l == 0)
                || (dc < 0 && c == 0)
                || (dl > 0 && l + 1 == max_l)
                || (dc > 0 && c + 1 == max_c)
            {
            } else {
                let new_l = (dl + (l as isize)) as usize;
                let new_c = (dc + (c as isize)) as usize;
                if !dejavu.contains(&(new_l, new_c))
                    && grid_reg[new_l][new_c] == RegCell::Region(None)
                {
                    grid_reg[new_l][new_c] = RegCell::Region(Some(reg_nbr));
                    dejavu.push((new_l, new_c));
                    queue.push_back((new_l, new_c));
                }
            }
        }
    }
}
////////////////////////////////////////////////////////////////////////////
