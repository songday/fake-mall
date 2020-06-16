use std::rc::Rc;

// use html5ever::parse_document;
use html5ever::driver::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::dto::{Item};

async fn get_html(url: &str) -> Result<Vec<u8>, ()> {
    let r = surf::get(url).await;
    if let Err(e) = r {
        dbg!(e);
        return Err(())
    }
    let r = r.unwrap().body_bytes().await;
    match r {
        Ok(body) => {
            Ok(body)
        },
        Err(e) => {
            dbg!(e);
            Err(())
        }
    }
}

fn walk(handle: &Handle, items: &mut Vec<Item>, item: Rc<Option<Item>>) {
    let node = handle;
    // FIXME: don't allocate
    // print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => println!("#Document"),

        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { ref contents } => {
            //println!("#text: {}", escape_default(&contents.borrow()))
            println!("#text: {}", &contents.borrow())
        },

        NodeData::Comment { ref contents } => println!("<!-- {} -->", /*escape_default(contents)*/contents),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(child, items, item.clone());
    }
}

pub async fn get_items(url: &str) -> Vec<Item> {
    let r = get_html(url).await;
    if let Err(e) = r {
        dbg!("{}", e);
        return vec![];
    }
    let mut items: Vec<Item> = vec![];
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut r.unwrap().as_slice())
        .unwrap();
    walk(&dom.document, &mut items, Rc::new(None));

    items
}