//
// pub(crate) async fn sse_handler(
//     Extension(user): Extension<User>,
//     State(state): State<AppState>,
// ) -> Sse<impl Stream<Item=Result<Event, Infallible>>> {
//     let user_id = user.id as u64;
//     let users = &state.users;
//
//     let rx = if let Some(tx) = users.get(&user_id) {
//         tx.subscribe()
//     } else {
//         let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);
//         state.users.insert(user_id, tx);
//         rx
//     };
//     info!("User {} subscribed", user_id);
//
//     let stream = BroadcastStream::new(rx).filter_map(|v| v.ok()).map(|v| {
//         let name = match v.as_ref() {
//             AppEvent::NewChat(_) => "NewChat",
//             AppEvent::AddToChat(_) => "AddToChat",
//             AppEvent::RemoveFromChat(_) => "RemoveFromChat",
//             AppEvent::NewMessage(_) => "NewMessage",
//         };
//         let v = serde_json::to_string(&v).expect("Failed to serialize event");
//         debug!("Sending event {}: {:?}", name, v);
//         Ok(Event::default().data(v).event(name))
//     });
//
//     Sse::new(stream).keep_alive(
//         axum::response::sse::KeepAlive::new()
//             .interval(Duration::from_secs(1))
//             .text("keep-alive-text"),
//     )
// }
//
//
// fn main() {
//     let cors = CorsLayer::new()
//         // allow `GET` and `POST` when accessing the resource
//         .allow_methods([
//             Method::GET,
//             Method::POST,
//             Method::PATCH,
//             Method::DELETE,
//             Method::PUT,
//         ])
//         .allow_origin(cors::Any)
//         .allow_headers(cors::Any);
//
//     let app = Router::new()
//         .route("/events", get(sse_handler))
//         // .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
//         // .route("/", get(index_handler))
//         .layer(cors);
// }
//
//
