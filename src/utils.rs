pub fn push_buffer_to_collection<T>(collection: &mut Vec<T>, buffer: &mut String)
where
    T: From<String>,
{
    if !buffer.is_empty() {
        collection.push(T::from(buffer.to_string()));
        buffer.clear();
    }
}
