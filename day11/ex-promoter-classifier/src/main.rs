//! Day 11 — Exercise 7: trainable promoter classifier.

use burn::backend::{Autodiff, NdArray};
use burn::module::Module;
use burn::nn::conv::{Conv1d, Conv1dConfig};
use burn::nn::{Linear, LinearConfig};
#[allow(unused_imports)]
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
// Note: when you fill in `forward`, you will want
//     use burn::tensor::activation::relu;
use burn::tensor::backend::{AutodiffBackend, Backend};
use burn::tensor::{Device, Int, Tensor, TensorData};
use rand::{Rng, SeedableRng, rngs::StdRng};

type Train = Autodiff<NdArray>;
#[allow(dead_code)]
type Eval = NdArray;

#[derive(Module, Debug)]
pub struct PromoterNet<B: Backend> {
    conv: Conv1d<B>,
    fc: Linear<B>,
}

impl<B: Backend> PromoterNet<B> {
    pub fn new(device: &Device<B>, kernel: usize, channels: usize) -> Self {
        Self {
            conv: Conv1dConfig::new(4, channels, kernel)
                .with_bias(false)
                .init(device),
            fc: LinearConfig::new(channels, 1).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        // TODO:
        //   h = relu(self.conv.forward(x))           // [N, C, L-k+1]
        //   h = h.max_dim(2).squeeze_dim(2)          // [N, C]
        //   return self.fc.forward(h)                // [N, 1]
        let _ = x;
        todo!("Hint 1 in 07-promoter-classifier.qmd")
    }
}

fn train_step<B, O>(
    model: PromoterNet<B>,
    optim: &mut O,
    lr: f64,
    x: Tensor<B, 3>,
    y: Tensor<B, 2>,
) -> (PromoterNet<B>, f32)
where
    B: AutodiffBackend,
    O: Optimizer<PromoterNet<B>, B>,
{
    // TODO: hint 2.
    //   1. logits = model.forward(x)
    //   2. loss = (logits - y).powf_scalar(2.0).mean()
    //   3. loss_value: f32 = loss.clone().into_data().to_vec::<f32>().unwrap()[0]
    //   4. grads = loss.backward()
    //   5. grads = GradientsParams::from_grads(grads, &model)
    //   6. model = optim.step(lr, model, grads)
    let _ = (model, optim, lr, x, y);
    todo!("Hint 2 in 07-promoter-classifier.qmd")
}

// ---------- synthetic data --------------------------------------------------

fn seq_to_ints(seq: &str) -> Vec<i64> {
    seq.chars().map(|c| match c {
        'A' => 0, 'C' => 1, 'G' => 2, 'T' => 3,
        _ => panic!("bad base {c:?}"),
    }).collect()
}

fn random_seq(rng: &mut StdRng, l: usize) -> String {
    (0..l).map(|_| ['A', 'C', 'G', 'T'][rng.gen_range(0..4)]).collect()
}

fn planted_promoter(rng: &mut StdRng, l: usize, motif: &str, pos: usize) -> String {
    let mut s = random_seq(rng, l).into_bytes();
    for (i, b) in motif.bytes().enumerate() { s[pos + i] = b; }
    String::from_utf8(s).unwrap()
}

/// Build a balanced batch of `n` promoter + `n` non-promoter sequences. Returns
/// the one-hot tensor (`[2n, 4, L]`) and the labels tensor (`[2n, 1]`).
fn make_dataset<B: Backend>(
    n_per_class: usize, l: usize, seed: u64, device: &Device<B>,
) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut seqs: Vec<String> = Vec::new();
    let mut labels: Vec<f32> = Vec::new();

    for _ in 0..n_per_class {
        seqs.push(planted_promoter(&mut rng, l, "TATAAA", 10));
        labels.push(1.0);
    }
    for _ in 0..n_per_class {
        seqs.push(random_seq(&mut rng, l));
        labels.push(0.0);
    }

    let n = seqs.len();
    let flat: Vec<i64> = seqs.iter().flat_map(|s| seq_to_ints(s)).collect();
    let ints: Tensor<B, 2, Int> =
        Tensor::<B, 1, Int>::from_data(TensorData::new(flat, [n * l]), device).reshape([n, l]);
    let one_hot: Tensor<B, 3> = ints.one_hot(4).float().swap_dims(1, 2);

    let y: Tensor<B, 2> =
        Tensor::<B, 1>::from_data(TensorData::new(labels, [n]), device).reshape([n, 1]);

    (one_hot, y)
}

// ---------- main: train and report accuracy ---------------------------------

fn main() {
    let device = Default::default();
    let (x, y) = make_dataset::<Train>(50, 32, 0xC0FFEE, &device);

    let mut model = PromoterNet::<Train>::new(&device, 6, 8);
    let mut optim = AdamConfig::new().init();
    let lr = 1e-2;

    for step in 0..200 {
        let (m, loss) = train_step(model, &mut optim, lr, x.clone(), y.clone());
        model = m;
        if step % 20 == 0 { println!("step {step:3} loss {loss:.4}"); }
    }

    let (x_eval, y_eval) = make_dataset::<Train>(50, 32, 0xC0FFEE, &device);
    let logits = model.forward(x_eval);
    let preds = logits.greater_equal_elem(0.5).int().float();
    let acc = preds.equal(y_eval.int().float()).int().float().mean();
    let acc_value: f32 = acc.into_data().to_vec::<f32>().unwrap()[0];
    println!("training-set accuracy: {acc_value:.3}");
}

// ---------- tests -----------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_shape() {
        let device = Default::default();
        let model = PromoterNet::<Eval>::new(&device, 6, 8);
        let (x, _) = make_dataset::<Eval>(2, 32, 7, &device);
        let logits = model.forward(x);
        assert_eq!(logits.dims(), [4, 1]);
    }

    #[test]
    fn one_train_step_runs() {
        let device = Default::default();
        let model = PromoterNet::<Train>::new(&device, 6, 8);
        let mut optim = AdamConfig::new().init();
        let (x, y) = make_dataset::<Train>(4, 32, 1, &device);
        let (model, loss) = train_step(model, &mut optim, 1e-2, x, y);
        assert!(loss.is_finite(), "loss must be finite, got {loss}");
        // The model should still be a valid Module after one step.
        let _ = model;
    }

    #[test]
    fn training_reduces_loss() {
        let device = Default::default();
        let mut model = PromoterNet::<Train>::new(&device, 6, 8);
        let mut optim = AdamConfig::new().init();
        let (x, y) = make_dataset::<Train>(20, 32, 2, &device);

        let (m, loss0) = train_step(model, &mut optim, 1e-2, x.clone(), y.clone());
        model = m;
        for _ in 0..100 {
            let (m, _) = train_step(model, &mut optim, 1e-2, x.clone(), y.clone());
            model = m;
        }
        let (_, loss_n) = train_step(model, &mut optim, 1e-2, x, y);

        assert!(loss_n < loss0 * 0.5,
                "loss did not decrease meaningfully: {loss0} -> {loss_n}");
    }
}
