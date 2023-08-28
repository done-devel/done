use std::pin::Pin;

use crate::{
	models::{list::List, task::Task},
	task_service::TaskService,
};
use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use url::Url;

#[derive(Debug, Clone, Copy)]
pub struct Smart;

impl Smart {
	pub fn new() -> Self {
		Self
	}
}

#[async_trait]
#[allow(unused)]
impl TaskService for Smart {
	async fn handle_uri_params(&mut self, _uri: Url) -> Result<()> {
		Ok(())
	}

	fn login(&self) -> anyhow::Result<()> {
		Ok(())
	}

	fn logout(&self) -> anyhow::Result<()> {
		Ok(())
	}

	fn available(&self) -> bool {
		true
	}

	fn stream_support(&self) -> bool {
		false
	}

	async fn read_tasks(&mut self) -> Result<Vec<Task>> {
		Ok(vec![])
	}

	async fn read_tasks_from_list(
		&mut self,
		parent_list: String,
	) -> Result<Vec<Task>> {
		Ok(vec![])
	}

	fn get_task_stream(
		&mut self,
		_parent_list: String,
	) -> Pin<Box<dyn Stream<Item = Result<Task>> + Send + '_>> {
		todo!("This service does not implement streams")
	}

	async fn read_task(
		&mut self,
		task_list_id: String,
		task_id: String,
	) -> Result<Task> {
		Ok(Task::default())
	}

	async fn create_task(&mut self, task: Task) -> Result<()> {
		Ok(())
	}

	async fn update_task(&mut self, task: Task) -> Result<Task> {
		Ok(Task::default())
	}

	async fn delete_task(
		&mut self,
		list_id: String,
		task_id: String,
	) -> Result<()> {
		Ok(())
	}

	async fn read_lists(&mut self) -> Result<Vec<List>> {
		Ok(vec![])
	}

	fn get_task_list_stream(
		&mut self,
	) -> Pin<Box<dyn Stream<Item = Result<List>> + Send + '_>> {
		todo!("This service does not implement streams")
	}

	async fn read_list(&mut self, id: String) -> Result<List> {
		Ok(List::default())
	}

	async fn create_list(&mut self, list: List) -> Result<List> {
		Ok(List::default())
	}

	async fn update_list(&mut self, list: List) -> Result<()> {
		Ok(())
	}

	async fn delete_list(&mut self, id: String) -> Result<()> {
		Ok(())
	}
}
