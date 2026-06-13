use swayipc::{Connection, Event, EventType, WindowChange};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn focused() -> Res<i64> {
    let mut conn = Connection::new()?;

    conn.get_tree()?
        .find_focused(|n| n.focused)
        .map(|n| n.id)
        .ok_or_else(|| "no focus".into())
}

fn focus(id: i64) -> Res<()> {
    let mut conn = Connection::new()?;

    conn.run_command(format!("[con_id={}] focus", id))?;

    Ok(())
}

fn main() -> Res<()> {
    let mut current = focused()?;
    let mut previous: Option<i64> = None;

    let events = Connection::new()?.subscribe(&[EventType::Window, EventType::Binding])?;

    for event in events {
        match event? {
            Event::Window(ev) => {
                if ev.change == WindowChange::Focus {
                    let id = ev.container.id;

                    if id != current {
                        previous = Some(current);
                        current = id;
                    }
                }
            }

            Event::Binding(ev) => {
                let binding = &ev.binding;

                //if binding.symbol.as_deref() == Some("Tab") && binding.event_state_mask.iter().any(|x| x == "Mod4")
                // see BindingEvent
                if binding.command == "nop" {
                    if let Some(prev) = previous {
                        focus(prev)?;
                        std::mem::swap(&mut current, previous.as_mut().unwrap());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
