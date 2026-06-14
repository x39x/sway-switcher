use swayipc::{Connection, Event, EventType, WindowChange};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const HISTORY_SIZE: usize = 10;

fn focus(id: i64) -> Res<()> {
    let mut conn = Connection::new()?;

    let result = conn.run_command(format!("[con_id={}] focus", id))?;

    if result.iter().any(|r| !r.is_ok()) {
        return Err(format!("failed to focus {}", id).into());
    }

    Ok(())
}

fn push_history(history: &mut Vec<i64>, id: i64) {
    history.retain(|x| *x != id);
    history.insert(0, id);

    if history.len() > HISTORY_SIZE {
        history.truncate(HISTORY_SIZE);
    }
}

fn main() -> Res<()> {
    let mut history: Vec<i64> = Vec::new();

    // init
    {
        let mut conn = Connection::new()?;

        if let Some(node) = conn.get_tree()?.find_focused(|n| n.focused) {
            history.push(node.id);
        }
    }

    let events = Connection::new()?.subscribe(&[EventType::Window, EventType::Binding])?;

    for event in events {
        match event? {
            Event::Window(ev) => match ev.change {
                WindowChange::Focus => {
                    push_history(&mut history, ev.container.id);
                }

                // clean history
                WindowChange::Close => {
                    history.retain(|x| *x != ev.container.id);
                }

                _ => {}
            },

            Event::Binding(ev) => {
                // see BindingEvent
                if ev.binding.command == "nop" {
                    for &id in history.iter().skip(1) {
                        if focus(id).is_ok() {
                            break;
                        }
                    }
                }
            }

            _ => {}
        }
    }

    Ok(())
}
