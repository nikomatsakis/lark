use crate::full_inference::analysis::dump::DumpCx;
use crate::full_inference::analysis::initialization::Initialization;
use crate::full_inference::analysis::kind_inference::KindInference;
use crate::full_inference::analysis::AnalysisIr;
use crate::full_inference::analysis::Node;
use crate::full_inference::analysis::Path;
use crate::full_inference::Perm;
use crate::HirLocation;
use crate::TypeCheckDatabase;
use datafrog::Iteration;
use datafrog::Relation;
use datafrog::RelationLeaper;
use polonius::facts::AllFacts;

crate struct LoanCheck {
    crate error_move_of_imprecise_path: Relation<(Node, ())>,
    crate error_access_to_uninitialized_path: Relation<(Path, Node)>,
}

struct LoanData {
    perm: Perm,
    path: Path,
    node: Node,
}

impl LoanCheck {
    /// Executes the **loan** analysis. Right now we implement this
    /// using polonius. This involves a bit of translation.
    crate fn new(
        cx: &DumpCx<'_, impl TypeCheckDatabase>,
        analysis_ir: &AnalysisIr,
        kind_inference: &KindInference,
        initialization: &Initialization,
    ) -> Self {
        // First step: identify the **loans**. A loan in our terms is an access
        // that is not a move -- i.e., where the resulting permission is not `own`.
        let owned = &kind_inference.owned;
        let loans: Relation<LoanData> = Relation::from_antijoin(&access_by_perm, owned, |&perm, &(path, node)| LoanData { perm, path, node });
        let loans: IndexVec<Loan, LoanData> = 


        // `borrow_region(R, L, P)` from polonius relates a loan L that
        // occurs at some point to the borrow region R. We don't have loans
        // identified yet, but we know all the **accesses**, some of which are
        // loans. Let's construct a 
        //
        // We'll use the pair of a `Perm x Path` to identify a loan for now.
        let borrow_region: Vec<(Perm, (Perm, Path), 

        let polonius_facts = AllFacts {};
    }
}
