#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    /// Retrieves all notes stored in the contract instance.
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// Creates a new note and appends it to the storage vector.
    pub fn create_note(env: Env, title: String, content: String) -> String {
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        let note = Note {
            id: env.prng().gen::<u64>(),
            title,
            content,
        };

        notes.push_back(note);
        env.storage().instance().set(&NOTE_DATA, &notes);

        String::from_str(&env, "Notes berhasil ditambahkan")
    }

    /// Deletes a note by its unique u64 ID.
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&NOTE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if let Some(note) = notes.get(i) {
                if note.id == id {
                    notes.remove(i);
                    env.storage().instance().set(&NOTE_DATA, &notes);
                    return String::from_str(&env, "Berhasil hapus notes");
                }
            }
        }

        String::from_str(&env, "Notes tidak ditemukan")
    }
}

