use atty::Stream;

extern crate atty;

pub fn is_interactive() -> bool {
    _is_interactive(Stream::Stdout) || _is_interactive(Stream::Stderr)
}

fn _is_interactive(stream: Stream) -> bool {
    let is_ci = std::env::var_os("CI").is_some();
    let is_dumb_term = match std::env::var("TERM") {
        Ok(string) => {
            string == "dumb"
        },
        Err(_error) => {
            // "env.TERM" doesn't exist
            // we can treat this as interactive
            false 
        }
    };

    atty::is(stream) && !is_ci && !is_dumb_term
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interactive() {
        assert_eq!(is_interactive(), true);
    }

    #[test]
    fn test_is_interactive_ci() {
        std::env::set_var("CI", "true");
        assert_eq!(is_interactive(), false);
    }

    #[test]
    fn test_is_interactive_dumb_term() {
        std::env::set_var("TERM", "dumb");
        assert_eq!(is_interactive(), false);
    }

    #[test]
    fn test_is_interactive_ci_dumb_term() {
        std::env::set_var("CI", "true");
        std::env::set_var("TERM", "dumb");
        assert_eq!(is_interactive(), false);
    }

    #[test]
    fn test_is_interactive_ci_dumb_term_stdout() {
        std::env::set_var("CI", "true");
        std::env::set_var("TERM", "dumb");
        assert_eq!(_is_interactive(Stream::Stdout), false);
    }

    #[test]
    fn test_is_interactive_ci_dumb_term_stderr() {
        std::env::set_var("CI", "true");
        std::env::set_var("TERM", "dumb");
        assert_eq!(_is_interactive(Stream::Stderr), false);
    }

    #[test]
    fn test_is_interactive_ci_stdout() {
        std::env::set_var("CI", "true");
        assert_eq!(_is_interactive(Stream::Stdout), false);
    }

    #[test]
    fn test_is_interactive_ci_stderr() {
        std::env::set_var("CI", "true");
        assert_eq!(_is_interactive(Stream::Stderr), false);
    }

    #[test]
    fn test_is_interactive_dumb_term_stdout() {
        std::env::set_var("TERM", "dumb");
        assert_eq!(_is_interactive(Stream::Stdout), false);
    }

    #[test]
    fn test_is_interactive_dumb_term_stderr() {
        std::env::set_var("TERM", "dumb");
        assert_eq!(_is_interactive(Stream::Stderr), false);
    }
    
}