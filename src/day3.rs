use std::fs;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Symbol {
    char: char,
    point: Point,
}

#[derive(Debug)]
struct Rect {
    tl: Point,
    br: Point,
}

impl Rect {
    fn expand(&self) -> Self {
        Rect {
            tl: Point {
                x: self.tl.x - 1,
                y: self.tl.y - 1,
            },
            br: Point {
                x: self.br.x + 1,
                y: self.br.y + 1,
            },
        }
    }

    // didn't need this
    fn _masked(&self, mask: &Self) -> Self {
        Rect {
            tl: Point {
                x: self.tl.x.max(mask.tl.x),
                y: self.tl.y.max(mask.tl.y),
            },
            br: Point {
                x: self.br.x.min(mask.br.x),
                y: self.br.y.min(mask.br.y),
            },
        }
    }

    fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.tl.x && point.x <= self.br.x && point.y >= self.tl.y && point.y <= self.br.y
    }

    fn overlaps_rect(&self, other: &Rect) -> bool {
        self.tl.x <= other.br.x
            && self.tl.y <= other.br.y
            && self.br.x >= other.tl.x
            && self.br.y >= other.tl.y
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Rect};

    #[test]
    fn test_rect_contains() {
        let r = Rect {
            tl: Point { x: 0, y: 0 },
            br: Point { x: 0, y: 0 },
        };
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 1 };
        assert!(r.contains_point(&p1));
        assert!(!r.contains_point(&p2));
    }

    #[test]
    fn test_rect_collide() {
        let r1 = Rect {
            tl: Point { x: 0, y: -2 },
            br: Point { x: 0, y: 2 },
        };
        let r2 = Rect {
            tl: Point { x: -2, y: 0 },
            br: Point { x: 2, y: 0 },
        };
        assert!(r1.overlaps_rect(&r2));
    }
}

#[derive(Debug)]
struct Schematic {
    rect: Rect,
    symbols: Vec<Symbol>,
    parts: Vec<Part>,
}

impl Schematic {
    fn from_input_string(input: String) -> Self {
        #[derive(PartialEq, Debug)]
        enum TokenKind {
            Period,
            Symbol,
            PartNumber,
        }

        #[derive(Debug)]
        struct Token {
            kind: TokenKind,
            string: String,
            index: usize,
        }

        impl Token {
            fn new(index: usize, kind: TokenKind) -> Self {
                Token {
                    index,
                    kind,
                    string: String::new(),
                }
            }
        }

        let rect = {
            let height = input.lines().count();
            let width = input.lines().next().unwrap().len();
            Rect {
                tl: Point { x: 0, y: 0 },
                br: Point {
                    x: width as i32,
                    y: height as i32,
                },
            }
        };

        let mut tokens = Vec::<Token>::new();
        let mut lines = input.lines().enumerate();
        let mut token: Option<Token> = None;

        while let Some((y, line)) = lines.next() {
            let mut chars = line.chars().enumerate();

            while let Some((x, char)) = chars.next() {
                let index = y * rect.br.x as usize + x;

                let kind = if char.is_ascii_digit() {
                    TokenKind::PartNumber
                } else if char == '.' {
                    TokenKind::Period
                } else {
                    TokenKind::Symbol
                };

                // should we make a new token?
                let new_token = !token.as_ref().is_some_and(|token| token.kind == kind);

                if new_token {
                    // if there's already a token, take it and push it to the list, create a new token
                    if let Some(token) = token.take() {
                        tokens.push(token);
                    }
                    token = Some(Token::new(index, kind));
                }

                // should always be a token by here, should be better way to do this
                if let Some(ref mut token) = token {
                    token.string.push(char);
                }
            }
        }

        let mut symbols = Vec::<Symbol>::new();
        let mut parts = Vec::<Part>::new();

        for token in tokens {
            let tl = Point {
                x: token.index as i32 % rect.br.x,
                y: token.index as i32 / rect.br.x,
            };

            if token.kind == TokenKind::Symbol {
                assert_eq!(token.string.len(), 1);
                symbols.push(Symbol {
                    point: tl,
                    char: token.string.chars().nth(0).unwrap(),
                });
            } else if token.kind == TokenKind::PartNumber {
                let mut br = tl.clone();
                br.x += token.string.len() as i32 - 1;

                let rect = Rect { tl, br };

                parts.push(Part {
                    string: token.string.clone(),
                    rect,
                });
            }
        }

        Schematic {
            rect,
            parts,
            symbols,
        }
    }
}

#[derive(Debug)]
struct Part {
    string: String,
    rect: Rect,
}

pub fn part1() -> u32 {
    let input = fs::read_to_string("input/day3/input.txt").unwrap();
    let schematic = Schematic::from_input_string(input);

    dbg!(&schematic.parts.len(), &schematic.symbols.len());

    let mut solution = 0u32;

    for part in &schematic.parts {
        for symbol in &schematic.symbols {
            let search_rect = part.rect.expand();
            if search_rect.contains_point(&symbol.point) {
                println!("part: {}, symbol: {}", &part.string, &symbol.char);
                let part_number_uint: u32 = part.string.parse().unwrap();
                solution += part_number_uint;
            }
        }
    }

    return solution;
}

pub fn part2() -> u32 {
    let input = fs::read_to_string("input/day3/input.txt").unwrap();
    let schematic = Schematic::from_input_string(input);

    dbg!(&schematic.parts.len(), &schematic.symbols.len());

    let mut solution = 0u32;

    for symbol in &schematic.symbols {
        if symbol.char != '*' {
            continue;
        }

        let search_rect = Rect::expand(&Rect {
            tl: symbol.point.clone(),
            br: symbol.point.clone(),
        });

        let mut adjacent_parts = Vec::<&Part>::new();

        for part in &schematic.parts {
            if search_rect.overlaps_rect(&part.rect) {
                dbg!(&search_rect, &part.rect);
                adjacent_parts.push(&part);
            }
        }

        if adjacent_parts.len() == 2 {
            let part1_number: u32 = adjacent_parts[0].string.parse().unwrap();
            let part2_number: u32 = adjacent_parts[1].string.parse().unwrap();
            solution += part1_number * part2_number;
        }
    }

    return solution;
}
