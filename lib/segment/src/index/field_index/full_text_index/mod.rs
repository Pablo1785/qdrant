mod inverted_index;
mod inverted_index_on_disk;
mod posting_list;
mod postings_iterator;
pub mod text_index;
mod tokenizers;
pub use inverted_index::InvertedIndex;

#[cfg(test)]
mod tests;
