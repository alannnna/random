// EXERCISE 5: Two structs with different lifetimes that coexist.
// A `View` borrows from a `Buffer`. They have different lifetimes.
// Tie them together correctly so a View can't outlive its Buffer.
// Run: cargo test --bin ex05_two_lifetimes -p module_10_zero_copy

#[derive(Debug)]
struct Buffer {
    data: String,
}

impl Buffer {
    fn new(data: &str) -> Buffer {
        Buffer { data: data.to_string() }
    }
}

// BUG: View borrows from a Buffer — it needs a lifetime that ties it to the Buffer's data
#[derive(Debug)]
struct View {
    slice: &str,   // compile error: missing lifetime
    start: usize,
    end: usize,
}

impl View {
    // BUG: lifetime ties are missing — the View should not outlive the Buffer
    fn from_buffer(buf: &Buffer, start: usize, end: usize) -> View {
        let end = end.min(buf.data.len());
        View {
            slice: &buf.data[start..end],
            start,
            end,
        }
    }

    fn content(&self) -> &str {
        self.slice
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

fn main() {
    let buf = Buffer::new("hello world, this is a buffer");
    let view = View::from_buffer(&buf, 6, 11);
    println!("View: '{}' (len={})", view.content(), view.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view() {
        let buf = Buffer::new("hello world");
        let view = View::from_buffer(&buf, 6, 11);
        assert_eq!(view.content(), "world");
        assert_eq!(view.len(), 5);
    }

    #[test]
    fn test_view_full() {
        let buf = Buffer::new("rust");
        let view = View::from_buffer(&buf, 0, 4);
        assert_eq!(view.content(), "rust");
    }

    #[test]
    fn test_view_clamp() {
        let buf = Buffer::new("hi");
        let view = View::from_buffer(&buf, 0, 100); // end clamped to buf length
        assert_eq!(view.content(), "hi");
    }
}
