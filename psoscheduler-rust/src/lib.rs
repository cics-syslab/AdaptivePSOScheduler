use std::sync::atomic::Ordering;
use ndarray::Array;
use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::time::{Instant, Duration};
use std::env;

use crate::simulation::task::Task;

pub mod simulation {
  pub mod task;
  pub mod workload;
  pub mod virtual_machine;
  pub mod data_center;
}

pub mod pso {
  pub mod particle;
  pub mod pso_swarm;
}

pub mod utils {
  pub mod utilities;
}

pub fn run_pso_algorithm(workload: simulation::workload::Workload, data_center: simulation::data_center::DataCenter) {
  let mut swarm = pso::pso_swarm::PSOSwarm::new(workload, data_center);
  swarm.run_pso_algorithm();
}

pub fn reset_workload_id_counter() {
  simulation::task::TASK_ID_COUNTER.store(0, Ordering::Relaxed);
}

pub fn reset_data_center_id_counter() {
  simulation::virtual_machine::VIRTUAL_MACHINE_ID_COUNTER.store(0, Ordering::Relaxed);
}

pub fn build_test1_data_center() -> simulation::data_center::DataCenter {
  reset_data_center_id_counter();

  let mut data_center = simulation::data_center::DataCenter::new();

  let vm = simulation::virtual_machine::VirtualMachine::new(100, 500.);
  data_center.add_virtual_machine(vm);

  data_center
}

pub fn build_test1_workload() -> simulation::workload::Workload {
  reset_workload_id_counter();

  let mut workload = simulation::workload::Workload::new();

  let task = simulation::task::Task::new(10);
  workload.add_task(task);

  workload
}

pub fn build_test2_data_center() -> simulation::data_center::DataCenter {
  reset_data_center_id_counter();

  let mut data_center = simulation::data_center::DataCenter::new();

  let vm1 = simulation::virtual_machine::VirtualMachine::new(50, 500.);
  let vm2 = simulation::virtual_machine::VirtualMachine::new(100, 500.);
  data_center.add_virtual_machine(vm1);
  data_center.add_virtual_machine(vm2);

  data_center
}

pub fn build_test2_workload() -> simulation::workload::Workload {
  build_test1_workload()
}

pub fn build_test3_data_center() -> simulation::data_center::DataCenter {
  reset_data_center_id_counter();

  let n_vms: usize = 36;
  let vm_mips_high: usize = 100;
  let vm_mips_low: usize = 50;

  let vm_high_id: usize = utils::utilities::get_random_integer(0, n_vms);

  let mut data_center = simulation::data_center::DataCenter::new();
  
  for i in 0..n_vms {
    if i == vm_high_id {
      let vm = simulation::virtual_machine::VirtualMachine::new(vm_mips_high as u32, 500.);
      data_center.add_virtual_machine(vm);
    } else {
      let vm = simulation::virtual_machine::VirtualMachine::new(vm_mips_low as u32, 500.);

      data_center.add_virtual_machine(vm);
    }
  }

  data_center
}

pub fn build_test3_workload() -> simulation::workload::Workload {
  build_test1_workload()
}

pub fn build_test4_data_center() -> simulation::data_center::DataCenter {
  reset_data_center_id_counter();

  let n_vms: usize = 36;
  let vm_mips_high: usize = 100;
  let vm_mips_low: usize = 20;

  let mut data_center = simulation::data_center::DataCenter::new();

  for _ in 0..n_vms {
    let mips: u32 = utils::utilities::get_random_integer(vm_mips_low, vm_mips_high) as u32;
    let vm = simulation::virtual_machine::VirtualMachine::new(mips, 500.);
    data_center.add_virtual_machine(vm);
  }

  data_center
}

pub fn build_test4_workload() -> simulation::workload::Workload {
  build_test1_workload()
}

pub fn build_test5_data_center() -> simulation::data_center::DataCenter {
  build_test4_data_center()
}

pub fn build_test5_workload() -> simulation::workload::Workload {
  reset_workload_id_counter();

  let n_tasks: usize = 5;
  let task_mi: u32 = 10;

  let mut workload = simulation::workload::Workload::new();

  for _ in 0..n_tasks {
    let task = simulation::task::Task::new(task_mi);
    workload.add_task(task);
  }

  workload
}

pub fn build_test6_data_center() -> simulation::data_center::DataCenter {
  build_test4_data_center()
}

pub fn build_test6_workload() -> simulation::workload::Workload {
  reset_workload_id_counter();

  let n_tasks: usize = 2;
  let task_mi_low: usize = 5;
  let task_mi_high: usize = 20;

  let mut workload = simulation::workload::Workload::new();

  for _ in 0..n_tasks {
    let mi = utils::utilities::get_random_integer(task_mi_low, task_mi_high) as u32;
    let task = simulation::task::Task::new(mi);
    workload.add_task(task);
  }

  workload
}

pub fn build_test11_data_center() -> simulation::data_center::DataCenter {
  let n_vms: usize = 100;
  let vm_mips_low: usize = 1000;
  let vm_mips_high: usize = 5000;
  let watts_low: f32 = 200.;
  let watts_high: f32 = 1000.;

  let mut data_center = simulation::data_center::DataCenter::new();

  for _ in 0..n_vms {
    let mi = utils::utilities::get_random_integer(vm_mips_low, vm_mips_high) as u32;
    let asj = utils::utilities::get_random_float(watts_low, watts_high);
    let vm = simulation::virtual_machine::VirtualMachine::new(mi, asj);
    data_center.add_virtual_machine(vm);
  }

  simulation::virtual_machine::VIRTUAL_MACHINE_ID_COUNTER.store(0, Ordering::Relaxed);

  data_center
}

pub fn build_test11_workload() -> simulation::workload::Workload {
  let n_tasks: usize = 800;
  let task_mi_low: usize = 1000;
  let task_mi_high: usize = 4000;

  let mut workload = simulation::workload::Workload::new();

  for _ in 0..n_tasks {
    let mi = utils::utilities::get_random_integer(task_mi_low, task_mi_high) as u32;
    let task = simulation::task::Task::new(mi);
    workload.add_task(task);
  }

  simulation::task::TASK_ID_COUNTER.store(0, Ordering::Relaxed);

  workload
}

pub fn build_basic_workload() -> simulation::workload::Workload {
  let t = simulation::task::Task::new(500);
  let t2 = simulation::task::Task::new(600);
  let mut wk = simulation::workload::Workload::new();
  wk.add_task(t);
  wk.add_task(t2);

  simulation::task::TASK_ID_COUNTER.store(0, Ordering::Relaxed);

  wk
}

pub fn build_basic_data_center() -> simulation::data_center::DataCenter {
  let vm = simulation::virtual_machine::VirtualMachine::new(200, 300.0);
  let vm2 = simulation::virtual_machine::VirtualMachine::new(400, 500.0);
  let mut dc = simulation::data_center::DataCenter::new();
  dc.add_virtual_machine(vm);
  dc.add_virtual_machine(vm2);

  simulation::virtual_machine::VIRTUAL_MACHINE_ID_COUNTER.store(0, Ordering::Relaxed);

  dc
}

pub fn basic_tests() {
  println!("Hello, world!");
  let t = simulation::task::Task::new(9);
  let t2 = simulation::task::Task::new(19);
  println!("{} {}", t, t2);
  let mut wk = simulation::workload::Workload::new();
  wk.add_task(t);
  wk.add_task(t2);
  println!("{}", wk);
  let vm = simulation::virtual_machine::VirtualMachine::new(95, 5.263158);
  let vm2 = simulation::virtual_machine::VirtualMachine::new(99, 5.050505);
  println!("{} {}", vm, vm2);
  let mut dc = simulation::data_center::DataCenter::new();
  dc.add_virtual_machine(vm);
  dc.add_virtual_machine(vm2);
  println!("{}", dc);
  let assignee = dc.get_min_eet_virtual_machine(&wk.tasks[0]);
  println!("{}", assignee);
  dc.add_execution_time_to_virtual_machine(&wk.tasks[0], 0);
  dc.add_execution_time_to_virtual_machine(&wk.tasks[1], 1);
  let obj = dc.compute_objective();
  println!("{:?}", dc.virtual_machine_ready_time);
  println!("Objective: {}", obj);
  println!("Makespan: {}", dc.compute_makespan());
  println!("Throughput: {}", dc.compute_throughput());
  println!("TEC: {}", dc.compute_energy_consumption_kwh());
  let t3 = simulation::task::Task::new(100);
  let t4 = simulation::task::Task::new(50);
  wk.add_task(t3);
  wk.add_task(t4);
  for t in wk.get_sorted_tasks().iter() {
    println!("{}", t);
  }
}

pub fn run_test_pso_main() {
  let args: Vec<String> = env::args().collect();

  let test_id = args[1].clone();

  let workload = match test_id.as_str() {
    "1" => build_test1_workload(),
    "2" => build_test2_workload(),
    "3" => build_test3_workload(),
    "4" => build_test4_workload(),
    "5" => build_test5_workload(),
    "6" => build_test6_workload(),
    // "7" => build_test7_workload(),
    // "8" => build_test8_workload(),
    // "9" => build_test9_workload(),
    // "10" => build_test10_workload(),
    "11" => build_test11_workload(),
    _ => panic!("Test {} is not implemented", test_id),
  };

  let data_center = match test_id.as_str() {
    "1" => build_test1_data_center(),
    "2" => build_test2_data_center(),
    "3" => build_test3_data_center(),
    "4" => build_test4_data_center(),
    "5" => build_test5_data_center(),
    "6" => build_test6_data_center(),
    // "7" => build_test7_data_center(),
    // "8" => build_test8_data_center(),
    // "9" => build_test9_data_center(),
    // "10" => build_test10_data_center(),
    "11" => build_test11_data_center(),
    _ => panic!("Test {} is not implemented", test_id),
  };

  let mut swarm = pso::pso_swarm::PSOSwarm::new(workload, data_center);
  swarm.run_pso_algorithm();

  swarm.data_center.reset_virtual_machine_ready_time();
  
  for (task_id, vm_id) in swarm.global_best_task_vm_mapping.iter() {
    let task: &simulation::task::Task = swarm.workload.tasks.get(*task_id).unwrap();
    swarm.data_center.add_execution_time_to_virtual_machine(task, *vm_id);
  }

  println!("Global Best Objective: {:?}", swarm.global_best_objective);
  println!("Global Best Makespan: {:?}", swarm.data_center.compute_makespan());
  println!("Global Best Throughput: {:?} tasks/sec", swarm.data_center.compute_throughput());
  println!("Global Best Energy Consumption: {:?} kWh", swarm.data_center.compute_energy_consumption_kwh());

  println!("Saving objective history to objective_history.csv... ");
  // Save the cost history to local file
  utils::utilities::write_objective_history_to_csv(&swarm.get_particle_objective_history(), "./output_data/objective_history.csv");
}

pub fn ndarray_ops(arr1: &Array2<f32>, arr2: &Array2<f32>) {
  let _arr3 = (arr2 - arr1).mapv(|a| a * 5.);
}

pub fn benchmark_ndarray() {
  let arr1 :Array2<f32> = Array::random((1000, 100), Uniform::new(0., 10.));
  let arr2 :Array2<f32> = Array::random((1000, 100), Uniform::new(0., 10.));

  let mut total: Duration = Duration::new(0, 0);

  for _ in 0..1000 {
    let start = Instant::now();
    ndarray_ops(&arr1, &arr2);
    total += start.elapsed();
  }
  
  let ave = total / 1000;
  println!("{:?} average", ave);
}