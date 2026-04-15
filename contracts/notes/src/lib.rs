#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Address};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub deadline: u64, // Unix Timestamp
    pub status: Symbol, // e.g., "PENDING", "DONE", "EXPIRED"
}

#[contract]
pub struct SchedulerContract;

#[contractimpl]
impl SchedulerContract {
    /// Membuat jadwal tugas baru dengan deadline spesifik.
    pub fn add_schedule(env: Env, id: u64, description: String, deadline: u64) {
        let key = (symbol_short!("TASK"), id);
        
        if deadline <= env.ledger().timestamp() {
            panic!("Deadline harus di masa depan");
        }

        let task = Task {
            id,
            description,
            deadline,
            status: symbol_short!("PENDING"),
        };

        env.storage().persistent().set(&key, &task);
    }

    /// Memperbarui status tugas setelah verifikasi.
    pub fn complete_task(env: Env, id: u64) {
        let key = (symbol_short!("TASK"), id);
        let mut task: Task = env.storage().persistent().get(&key).expect("Tugas tidak ditemukan");

        if env.ledger().timestamp() > task.deadline {
            task.status = symbol_short!("EXPIRED");
        } else {
            task.status = symbol_short!("DONE");
        }

        env.storage().persistent().set(&key, &task);
    }

    /// Melihat detail jadwal berdasarkan ID.
    pub fn get_schedule(env: Env, id: u64) -> Task {
        let key = (symbol_short!("TASK"), id);
        env.storage().persistent().get(&key).expect("Jadwal tidak ditemukan")
    }
}