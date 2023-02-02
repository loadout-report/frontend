use js_sys::Math::{floor, random};
use log::info;
use yew::prelude::*;
use super::RollOption;

#[derive(Clone, PartialEq, Eq, Debug, Properties)]
pub struct RollProps {
    pub roll_options: Vec<RollOption>,
}

#[function_component(Roll)]
pub fn roll_component(props: &RollProps) -> Html {
    // let roll_options = use_context::<Vec<RollOption>>();
    let roll_options = props.roll_options.to_owned();
    let roll = use_state(|| None);
    let roll_callback_handle = roll.clone();
    let onclick = Callback::from(move |_| {
        let roll_handle = roll_callback_handle.clone();
        let random = random();
        let roll = floor(random * roll_options.len() as f64) as usize;
        let roll = roll_options.get(roll);
        let roll = (*roll.unwrap()).clone();
        roll_handle.set(Some(roll));
    });

    html!(
        <>
                <button {onclick}>{"Roll a gun"}</button>
                {
                    if let Some(roll) = (*roll).clone() {
                        html!(
                            <div>
                                <img style="width: 96px; height: 96px" src={roll.icon.clone()} alt={"exotic"} />
                                <h3>{roll.name.clone()}</h3>
                            </div>
                        )
                    } else {
                        html!(
                            <div><p>{"Not rolled yet"}</p></div>
                        )
                    }
                }
                </>
    )
}
