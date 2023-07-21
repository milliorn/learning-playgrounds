use rayon::{
    iter::plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
    prelude::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
};

type Data = i32;

#[derive(Debug)]
struct DataCollection {
    data: Vec<Data>,
}

struct ParDataIter<'a> {
    data_slice: &'a [Data],
}

struct DataProducer<'a> {
    data_slice: &'a [Data],
}

impl<'a> ParallelIterator for ParDataIter<'a> {
    type Item = &'a Data;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'iter> IndexedParallelIterator for ParDataIter<'iter> {
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let producer = DataProducer::from(self);
        callback.callback(producer)
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.data_slice.len()
    }
}

impl<'a> Producer for DataProducer<'a> {
    type Item = &'a Data;
    type IntoIter = std::slice::Iter<'a, Data>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_slice.iter()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.data_slice.split_at(index);
        (
            DataProducer { data_slice: left },
            DataProducer { data_slice: right },
        )
    }
}

impl<'a> From<ParDataIter<'a>> for DataProducer<'a> {
    fn from(iterator: ParDataIter<'a>) -> Self {
        Self {
            data_slice: iterator.data_slice,
        }
    }
}

impl DataCollection {}

impl<'a> IntoParallelIterator for &'a DataCollection {
    type Iter = ParDataIter<'a>;
    type Item = &'a Data;

    fn into_par_iter(self) -> Self::Iter {
        ParDataIter {
            data_slice: &self.data,
        }
    }
}
fn main() {
    let data: DataCollection = DataCollection {
        data: vec![1, 2, 3, 4],
    };

    println!("data = {:?}", data);

    let sum_of_squares: Data = data.par_iter().map(|x| x * x).sum();

    println!("sum = {}", sum_of_squares);
}
