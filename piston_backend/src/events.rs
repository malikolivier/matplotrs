use piston::input::*;

use matplotrs_backend as mb;
use self::mb::EventKind;

fn convert_button_state(state: ButtonState) -> mb::ButtonState {
    match state {
        ButtonState:: Press => mb::ButtonState::Press,
        ButtonState:: Release => mb::ButtonState::Release,
    }
}

pub fn convert_events(event: Event) -> Option<EventKind> {
    println!("{:?}", event);
    match event {
        Event::Input(input) => match input {
            Input::Button(args) => match args.button {
                Button::Keyboard(_key) => None, /* TODO Ignore for now! */
                Button::Mouse(button) => match button {
                    MouseButton::Left => Some(EventKind::Click(mb::ClickEvent {
                        state: convert_button_state(args.state),
                        button: mb::MouseButton::Left,
                    })),
                    MouseButton::Middle => Some(EventKind::Click(mb::ClickEvent {
                        state: convert_button_state(args.state),
                        button: mb::MouseButton::Middle,
                    })),
                    MouseButton::Right => Some(EventKind::Click(mb::ClickEvent {
                        state: convert_button_state(args.state),
                        button: mb::MouseButton::Right,
                    })),
                    _ => None,
                },
                Button::Controller(_) => None,
            }
            Input::Move(_motion) => None, /* TODO Ignore for now! */
            Input::Text(_) => None, /* TODO Ignore for now! */
            Input::Resize(w, h) => Some(EventKind::Resize(w, h)),
            Input::Focus(_focus) => None,
            Input::Cursor(_cursor) => None, /* TODO Ignore for now! */
            Input::Close(_) => Some(EventKind::Close),
        },
        Event::Loop(lp) => match lp {
            Loop::Render(_args) => Some(EventKind::Render),
            Loop::AfterRender(_args) => None,
            Loop::Update(args) => Some(EventKind::Update(args.dt)),
            Loop::Idle(_args) => None,
        }
        _ => unimplemented!(),
    }
}
