/* insertion_sort.rs
 *
 * Copyright (c) 2020 Aaron Erhardt
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use sort_benchmarks::{generate_test_array, insertion_sort_c, sort::insertion_sort_rs};

use criterion::{
    criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
    PlotConfiguration,
};

fn insertion_sort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insertion Sort");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("Rust", size), size, |b, &size| {
            b.iter_batched(
                || generate_test_array(size),
                |mut data| insertion_sort_rs(data.as_mut_slice()),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("C", size), size, |b, &size| {
            b.iter_batched(
                || generate_test_array(size),
                |mut data| unsafe { insertion_sort_c(data.as_mut_ptr(), data.len() as i32) },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(benches, insertion_sort_bench);
criterion_main!(benches);
