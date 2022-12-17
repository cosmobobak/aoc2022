use std::{cmp::Ordering, fmt::Display, iter::once};
use crate::util::MinMaxN;

#[derive(Debug, Clone)]
enum Element<'a> {
    Value(u8),
    List(ElementIterator<'a>),
}

impl Element<'_> {
    fn to_underlying(&self) -> &[u8] {
        match self {
            Element::Value(value) => panic!("Cannot convert value to underlying: {value}"),
            Element::List(iter) => iter.text,
        }
    }
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.clone();
        match s {
            Element::Value(value) => write!(f, "{value}"),
            Element::List(iter) => write!(
                f,
                "[{}]",
                iter.map(|e| format!("{e}")).collect::<Vec<_>>().join(",")
            ),
        }
    }
}

fn cmp(left: Element, right: Element) -> Ordering {
    match (left, right) {
        (Element::Value(left), Element::Value(right)) => left.cmp(&right),
        (Element::Value(left), Element::List(mut right)) => {
            right.next().map_or(Ordering::Greater, |rhead| {
                cmp(Element::Value(left), rhead)
                    .then_with(|| right.next().map_or(Ordering::Equal, |_| Ordering::Less))
            })
        }
        (Element::List(mut left), Element::Value(right)) => {
            left.next().map_or(Ordering::Less, |lhead| {
                cmp(lhead, Element::Value(right))
                    .then_with(|| left.next().map_or(Ordering::Equal, |_| Ordering::Greater))
            })
        }
        (Element::List(mut left), Element::List(mut right)) => loop {
            match (left.next(), right.next()) {
                (Some(next_left), Some(next_right)) => match cmp(next_left, next_right) {
                    Ordering::Equal => continue,
                    other => break other,
                },
                (Some(_), None) => break Ordering::Greater,
                (None, Some(_)) => break Ordering::Less,
                (None, None) => break Ordering::Equal,
            }
        },
    }
}

#[derive(Debug, Clone)]
struct ElementIterator<'a> {
    text: &'a [u8],
    index: usize,
    bracket_depth: usize,
}

impl<'a> ElementIterator<'a> {
    const fn new(text: &'a [u8]) -> Self {
        Self {
            text,
            index: 0,
            bracket_depth: 0,
        }
    }
}

impl<'a> Iterator for ElementIterator<'a> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        const OPEN: u8 = b'[';
        const CLOSE: u8 = b']';
        const SEP: u8 = b',';
        if self.index >= self.text.len() {
            return None;
        }

        match self.text[self.index] {
            OPEN => {
                self.index += 1;
                let start_index = self.index;
                let bracket_endpoint = self.bracket_depth;
                self.bracket_depth += 1;
                while self.bracket_depth > bracket_endpoint {
                    match self.text[self.index] {
                        OPEN => self.bracket_depth += 1,
                        CLOSE => self.bracket_depth -= 1,
                        _ => (),
                    }
                    self.index += 1;
                }
                Some(Element::List(Self::new(
                    &self.text[start_index..self.index - 1],
                )))
            }
            CLOSE => {
                self.index += 1;
                None
            }
            SEP => {
                self.index += 1;
                self.next()
            }
            _ => {
                let start_index = self.index;
                while self.index < self.text.len()
                    && self.text[self.index] != SEP
                    && self.text[self.index] != CLOSE
                {
                    self.index += 1;
                }
                Some(Element::Value(
                    std::str::from_utf8(&self.text[start_index..self.index])
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                ))
            }
        }
    }
}

pub fn task13() {
    let start = std::time::Instant::now();
    let input = include_str!("../tasks/task13.txt");

    let packets = input.split("\r\n\r\n");

    let mut sum = 0;
    for (i, packet) in packets.enumerate() {
        let (left, right) = packet.split_once("\r\n").unwrap();
        let left_bytes = left.as_bytes();
        let right_bytes = right.as_bytes();
        let left_parsed = ElementIterator::new(left_bytes).next().unwrap();
        let right_parsed = ElementIterator::new(right_bytes).next().unwrap();
        let order = cmp(left_parsed, right_parsed);
        assert_ne!(order, Ordering::Equal);
        if order == Ordering::Less {
            sum += i + 1;
        }
    }

    println!("Part 1: {sum}");

    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .chain(once("[[2]]"))
        .chain(once("[[6]]"))
        .map(str::as_bytes)
        .map(|l| ElementIterator::new(l).next().unwrap())
        .collect::<Vec<_>>();

    packets.sort_unstable_by(|a, b| cmp(a.clone(), b.clone()));

    let divider_packet_indexes = packets
        .iter()
        .enumerate()
        .max_n_ct_by::<2>(|(_, a), (_, b)| {
            const DIVIDER_PACKETS: [&[u8]; 2] = [b"[2]", b"[6]"];
            let a_is_dp = i8::from(DIVIDER_PACKETS.contains(&a.to_underlying()));
            let b_is_dp = i8::from(DIVIDER_PACKETS.contains(&b.to_underlying()));
            a_is_dp.cmp(&b_is_dp)
        })
        .map(|[(i1, _), (i2, _)]| (i1 + 1, i2 + 1))
        .unwrap();

    println!("Part 2: {}", divider_packet_indexes.0 * divider_packet_indexes.1);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
