/*
 * Copyright (C) 2025 Dustyn Gibb
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

fn sin_wave(input: u64) -> f64 {
    (input as f64).sin()
}

fn sin_scaled(input: u64, scale: f64) -> u64 {
    ((input as f64).sin() * scale) as u64
}

fn cos_wave(input: u64) -> f64 {
    (input as f64).cos()
}

fn cos_scaled(input: u64, scale: f64) -> u64 {
    ((input as f64).cos() * scale) as u64
}

fn tan_wave(input: u64) -> f64 {
    (input as f64).tan()
}

fn exp_wave(input: u64) -> f64 {
    (input as f64).exp()
}

fn log_wave(input: u64) -> f64 {
    (input as f64).ln()
}

fn sqrt_wave(input: u64) -> f64 {
    (input as f64).sqrt()
}

fn sqrt_scaled(input: u64, scale: f64) -> u64 {
    ((input as f64).sqrt() * scale) as u64
}

fn mod_wave(input: u64, modulus: u64) -> u64 {
    input % modulus
}

