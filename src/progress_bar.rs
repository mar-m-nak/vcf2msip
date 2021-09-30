pub struct ProgressBar {
    title: String,
    max: usize,
    pg: usize,
    width: usize,
    end_str_len: usize,
}

impl ProgressBar {

    /// Initialize progress bar
    pub fn new(title: &str, max: usize) -> Self {
        const _CONSOLE_W_MAX: usize = 80;
        let title = format!("> {} [", title);
        let end_str_len = max.to_string().len() * 2 + 3; // "] max/max"
        let width = _CONSOLE_W_MAX - title.len() - end_str_len;
        Self{ title, max, pg: 0, width, end_str_len }
    }

    /// Stretch this bar
    pub fn progress(&mut self) {
        self.pg += 1;
        let is_done = self.max <= self.pg;
        let per = if !is_done {
            (self.pg as f32 / self.max as f32 * self.width as f32).ceil() as usize
        } else {
            self.width
        };
        print!("\r \r{}", &self.title);
        for i in 1..self.width {
            let c = if i < per {"#"} else {" "};
            print!("{}", c);
        }
        if !is_done {
            print!("] {}/{}", self.pg, self.max);
        } else {
            let done_str = "] Done!";
            let spc = if self.end_str_len < done_str.len() {
                0
            } else {
                self.end_str_len - done_str.len()
            };
            println!("{msg}{:1$}", " ", spc, msg = done_str);
        }
    }
}