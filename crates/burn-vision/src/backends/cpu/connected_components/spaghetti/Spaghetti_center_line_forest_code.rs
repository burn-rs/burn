no_analyze! {{
use centerLabels::*;let mut label = entry;
while let Some(next) = (|label| -> Option<centerLabels> { match label {
        NODE_1=> {
        if img_row01[(c) as usize] > 0 {
            if img_row00[(c + 1) as usize] > 0 {
            return Some(NODE_2);
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
                return Some(cl_tree_7);
            }
        }
        else {
        return Some(NODE_3);
        }
                }
        NODE_3=> {
        if img_row00[(c + 1) as usize] > 0 {
        return Some(NODE_2);
        }
        else {
        return Some(NODE_4);
        }
                }
        NODE_4=> {
        if img_row01[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = solver.new_label();
            return Some(cl_tree_2);
        }
        else {
            img_labels_row00[c as usize] = 0;
            return Some(cl_tree_1);
        }
                }
        NODE_2=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_6);
        }
        else {
        return Some(NODE_5);
        }
                }
        NODE_5=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row11[(c) as usize] > 0 {
            return Some(NODE_6);
            }
            else {
                img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                return Some(cl_tree_5);
            }
        }
        else {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                return Some(cl_tree_4);
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
                return Some(cl_tree_3);
            }
        }
                }
        NODE_7=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_6);
        }
        else {
        return Some(NODE_8);
        }
                }
        NODE_9=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_4);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_3);
        }
                }
        NODE_10=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                return Some(cl_tree_11);
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                    if img_row12[(c) as usize] > 0 {
                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                        return Some(cl_tree_11);
                    }
                    else {
                        img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver), img_labels_row12[(c - 2) as usize], solver);
                        return Some(cl_tree_11);
                    }
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                    return Some(cl_tree_11);
                }
            }
        }
        else {
            if img_row00[(c + 1) as usize] > 0 {
                if img_row11[(c + 2) as usize] > 0 {
                    if img_row11[(c) as usize] > 0 {
                        if img_row12[(c + 1) as usize] > 0 {
                            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                            return Some(cl_tree_5);
                        }
                        else {
                            img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c) as usize], solver);
                            return Some(cl_tree_5);
                        }
                    }
                    else {
                        if img_row11[(c - 1) as usize] > 0 {
                            if img_row12[(c + 1) as usize] > 0 {
                                if img_row12[(c) as usize] > 0 {
                                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                                    return Some(cl_tree_5);
                                }
                                else {
                                    img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c - 2) as usize], solver);
                                    return Some(cl_tree_5);
                                }
                            }
                            else {
                                img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c - 2) as usize], solver);
                                return Some(cl_tree_5);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                            return Some(cl_tree_5);
                        }
                    }
                }
                else {
                    if img_row11[(c - 1) as usize] > 0 {
                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c - 2) as usize], solver);
                        return Some(cl_tree_8);
                    }
                    else {
                    return Some(NODE_9);
                    }
                }
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c - 2) as usize], solver);
                    return Some(cl_tree_12);
                }
                else {
                return Some(NODE_11);
                }
            }
        }
                }
        NODE_8=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                return Some(cl_tree_5);
            }
            else {
                if img_row11[(c) as usize] > 0 {
                    img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c) as usize], solver);
                    return Some(cl_tree_5);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
        }
        else {
        return Some(NODE_9);
        }
                }
        NODE_12=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                return Some(cl_tree_11);
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                return Some(NODE_13);
                }
                else {
                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                    return Some(cl_tree_11);
                }
            }
        }
        else {
            if img_row00[(c + 1) as usize] > 0 {
                if img_row11[(c + 2) as usize] > 0 {
                    if img_row11[(c) as usize] > 0 {
                    return Some(NODE_6);
                    }
                    else {
                        if img_row11[(c - 1) as usize] > 0 {
                        return Some(NODE_14);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                            return Some(cl_tree_5);
                        }
                    }
                }
                else {
                    if img_row11[(c) as usize] > 0 {
                        img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                        return Some(cl_tree_4);
                    }
                    else {
                        if img_row11[(c - 1) as usize] > 0 {
                            img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                            return Some(cl_tree_3);
                        }
                        else {
                            img_labels_row00[c as usize] = solver.new_label();
                            return Some(cl_tree_3);
                        }
                    }
                }
            }
            else {
                if img_row11[(c) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                    return Some(cl_tree_10);
                }
                else {
                    if img_row11[(c - 1) as usize] > 0 {
                        img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                        return Some(cl_tree_9);
                    }
                    else {
                        img_labels_row00[c as usize] = solver.new_label();
                        return Some(cl_tree_9);
                    }
                }
            }
        }
                }
        NODE_6=> {
        if img_row12[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
            return Some(cl_tree_5);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c + 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_15=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_11);
        }
        else {
            if img_row00[(c + 1) as usize] > 0 {
            return Some(NODE_5);
            }
            else {
                if img_row11[(c) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                    return Some(cl_tree_10);
                }
                else {
                    img_labels_row00[c as usize] = solver.new_label();
                    return Some(cl_tree_9);
                }
            }
        }
                }
        NODE_11=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_10);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_9);
        }
                }
        NODE_13=> {
        if img_row12[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_11);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c) as usize], img_labels_row12[(c - 2) as usize], solver);
            return Some(cl_tree_11);
        }
                }
        NODE_14=> {
        if img_row12[(c + 1) as usize] > 0 {
            if img_row12[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                return Some(cl_tree_5);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c + 2) as usize], img_labels_row12[(c - 2) as usize], solver);
                return Some(cl_tree_5);
            }
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c + 2) as usize], img_labels_row12[(c - 2) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_16=> {
        if img_row11[(c + 1) as usize] > 0 {
        return Some(NODE_17);
        }
        else {
        return Some(NODE_18);
        }
                }
        NODE_18=> {
        if img_row11[(c + 2) as usize] > 0 {
        return Some(NODE_19);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_8);
        }
                }
        NODE_17=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_6);
        }
        else {
            if img_row12[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                return Some(cl_tree_6);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                return Some(cl_tree_6);
            }
        }
                }
        NODE_20=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
            return Some(cl_tree_5);
        }
        else {
            if img_row12[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                return Some(cl_tree_5);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                return Some(cl_tree_5);
            }
        }
                }
        NODE_21=> {
        if img_row11[(c + 1) as usize] > 0 {
        return Some(NODE_22);
        }
        else {
            if img_row00[(c + 1) as usize] > 0 {
            return Some(NODE_18);
            }
            else {
                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                return Some(cl_tree_12);
            }
        }
                }
        NODE_23=> {
        if img_row11[(c + 2) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
            return Some(cl_tree_5);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_8);
        }
                }
        NODE_24=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_6);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_6);
        }
                }
        NODE_25=> {
        if img_row01[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_7);
        }
        else {
            img_labels_row00[c as usize] = solver.new_label();
            return Some(cl_tree_7);
        }
                }
        NODE_26=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                return Some(cl_tree_5);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                return Some(cl_tree_5);
            }
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_8);
        }
                }
        NODE_19=> {
        if img_row12[(c + 1) as usize] > 0 {
        return Some(NODE_20);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_27=> {
        if img_row01[(c) as usize] > 0 {
            if img_row00[(c + 1) as usize] > 0 {
                if img_row01[(c - 1) as usize] > 0 {
                return Some(NODE_7);
                }
                else {
                return Some(NODE_2);
                }
            }
            else {
            return Some(NODE_25);
            }
        }
        else {
        return Some(NODE_3);
        }
                }
        NODE_28=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_11);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_11);
        }
                }
        NODE_29=> {
        if img_row00[(c + 1) as usize] > 0 {
            if img_row11[(c + 1) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                return Some(cl_tree_6);
            }
            else {
                if img_row11[(c + 2) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
                    return Some(cl_tree_5);
                }
                else {
                    img_labels_row00[c as usize] = solver.new_label();
                    return Some(cl_tree_3);
                }
            }
        }
        else {
        return Some(NODE_4);
        }
                }
        NODE_30=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
                if img_row12[(c) as usize] > 0 {
                return Some(NODE_31);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                return Some(cl_tree_5);
            }
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_8);
        }
                }
        NODE_22=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_11);
        }
        else {
            if img_row12[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                return Some(cl_tree_11);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                return Some(cl_tree_11);
            }
        }
                }
        NODE_31=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c + 2) as usize];
            return Some(cl_tree_5);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_32=> {
        if img_row12[(c - 1) as usize] > 0 {
        return Some(NODE_17);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_6);
        }
                }
        NODE_33=> {
        if img_row12[(c - 1) as usize] > 0 {
        return Some(NODE_20);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_34=> {
        if img_row12[(c - 1) as usize] > 0 {
        return Some(NODE_22);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_11);
        }
                }
        NODE_35=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
                if img_row11[(c - 2) as usize] > 0 {
                return Some(NODE_33);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
            else {
                if img_row11[(c) as usize] > 0 {
                    if img_row11[(c - 2) as usize] > 0 {
                    return Some(NODE_36);
                    }
                    else {
                        img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c) as usize], solver);
                        return Some(cl_tree_5);
                    }
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
        }
        else {
            if img_row11[(c) as usize] > 0 {
                if img_row11[(c - 2) as usize] > 0 {
                return Some(NODE_37);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                    return Some(cl_tree_4);
                }
            }
            else {
                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                return Some(cl_tree_3);
            }
        }
                }
        NODE_37=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_4);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_4);
        }
                }
        NODE_38=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
                if img_row12[(c) as usize] > 0 {
                    if img_row11[(c - 2) as usize] > 0 {
                    return Some(NODE_31);
                    }
                    else {
                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                        return Some(cl_tree_5);
                    }
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                return Some(cl_tree_5);
            }
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            return Some(cl_tree_8);
        }
                }
        NODE_39=> {
        if img_row11[(c + 2) as usize] > 0 {
            if img_row12[(c + 1) as usize] > 0 {
            return Some(NODE_33);
            }
            else {
                if img_row11[(c) as usize] > 0 {
                return Some(NODE_36);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver);
                    return Some(cl_tree_5);
                }
            }
        }
        else {
            if img_row11[(c) as usize] > 0 {
            return Some(NODE_37);
            }
            else {
                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                return Some(cl_tree_3);
            }
        }
                }
        NODE_36=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c + 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_5);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c + 2) as usize], solver), img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_5);
        }
                }
        NODE_40=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            return Some(cl_tree_10);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            return Some(cl_tree_10);
        }
                }
cl_tree_0 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_0); } else { return Some(cl_break_1_0); } }
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_15);
                }
                else {
                    return Some(NODE_1);
                }
}
cl_tree_1 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_1); } else { return Some(cl_break_1_1); } }
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_12);
                }
                else {
                    return Some(NODE_1);
                }
}
cl_tree_2 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_2); } else { return Some(cl_break_1_2); } }
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_10);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_7);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        return Some(NODE_3);
                    }
                }
}
cl_tree_3 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_3); } else { return Some(cl_break_1_3); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row11[(c + 1) as usize] > 0 {
                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                        return Some(cl_tree_11);
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_23);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_12);
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row11[(c + 1) as usize] > 0 {
                                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                return Some(cl_tree_6);
                            }
                            else {
                                return Some(NODE_23);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        return Some(NODE_29);
                    }
                }
}
cl_tree_4 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_3); } else { return Some(cl_break_1_4); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row11[(c + 1) as usize] > 0 {
                        if img_row12[(c) as usize] > 0 {
                            return Some(NODE_28);
                        }
                        else {
                            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                            return Some(cl_tree_11);
                        }
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_30);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_12);
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row11[(c + 1) as usize] > 0 {
                                if img_row12[(c) as usize] > 0 {
                                    return Some(NODE_24);
                                }
                                else {
                                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                    return Some(cl_tree_6);
                                }
                            }
                            else {
                                return Some(NODE_30);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        return Some(NODE_29);
                    }
                }
}
cl_tree_5 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_3); } else { return Some(cl_break_1_5); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row11[(c + 1) as usize] > 0 {
                        img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                        return Some(cl_tree_11);
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_26);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_12);
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row11[(c + 1) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                return Some(cl_tree_6);
                            }
                            else {
                                return Some(NODE_26);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row11[(c + 1) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                return Some(cl_tree_6);
                            }
                            else {
                                if img_row11[(c + 2) as usize] > 0 {
                                    return Some(NODE_6);
                                }
                                else {
                                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                    return Some(cl_tree_4);
                                }
                            }
                        }
                        else {
                            return Some(NODE_4);
                        }
                    }
                }
}
cl_tree_6 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_3); } else { return Some(cl_break_1_6); } }
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_21);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_16);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        return Some(NODE_3);
                    }
                }
}
cl_tree_7 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_4); } else { return Some(cl_break_1_7); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        return Some(NODE_10);
                    }
                    else {
                        return Some(NODE_12);
                    }
                }
                else {
                    return Some(NODE_27);
                }
}
cl_tree_8 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_3); } else { return Some(cl_break_1_8); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row11[(c + 1) as usize] > 0 {
                        if img_row12[(c) as usize] > 0 {
                            if img_row11[(c - 2) as usize] > 0 {
                                return Some(NODE_28);
                            }
                            else {
                                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                return Some(cl_tree_11);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                            return Some(cl_tree_11);
                        }
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_38);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_12);
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row11[(c + 1) as usize] > 0 {
                                if img_row12[(c) as usize] > 0 {
                                    if img_row11[(c - 2) as usize] > 0 {
                                        return Some(NODE_24);
                                    }
                                    else {
                                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                        return Some(cl_tree_6);
                                    }
                                }
                                else {
                                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                    return Some(cl_tree_6);
                                }
                            }
                            else {
                                return Some(NODE_38);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            return Some(cl_tree_7);
                        }
                    }
                    else {
                        return Some(NODE_29);
                    }
                }
}
cl_tree_9 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_5); } else { return Some(cl_break_1_9); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        if img_row11[(c + 1) as usize] > 0 {
                            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                            return Some(cl_tree_11);
                        }
                        else {
                            if img_row00[(c + 1) as usize] > 0 {
                                return Some(NODE_8);
                            }
                            else {
                                return Some(NODE_11);
                            }
                        }
                    }
                    else {
                        return Some(NODE_15);
                    }
                }
                else {
                    return Some(NODE_27);
                }
}
cl_tree_10 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_6); } else { return Some(cl_break_1_10); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        if img_row11[(c + 1) as usize] > 0 {
                            return Some(NODE_34);
                        }
                        else {
                            if img_row00[(c + 1) as usize] > 0 {
                                return Some(NODE_39);
                            }
                            else {
                                if img_row11[(c) as usize] > 0 {
                                    return Some(NODE_40);
                                }
                                else {
                                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                    return Some(cl_tree_9);
                                }
                            }
                        }
                    }
                    else {
                        return Some(NODE_15);
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row01[(c - 1) as usize] > 0 {
                                if img_row11[(c + 1) as usize] > 0 {
                                    return Some(NODE_32);
                                }
                                else {
                                    return Some(NODE_39);
                                }
                            }
                            else {
                                return Some(NODE_2);
                            }
                        }
                        else {
                            return Some(NODE_25);
                        }
                    }
                    else {
                        return Some(NODE_3);
                    }
                }
}
cl_tree_11 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_7); } else { return Some(cl_break_1_11); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row00[(c - 1) as usize] > 0 {
                        return Some(NODE_21);
                    }
                    else {
                        if img_row01[(c - 1) as usize] > 0 {
                            return Some(NODE_21);
                        }
                        else {
                            if img_row11[(c + 1) as usize] > 0 {
                                if img_row11[(c) as usize] > 0 {
                                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                    return Some(cl_tree_11);
                                }
                                else {
                                    return Some(NODE_13);
                                }
                            }
                            else {
                                if img_row00[(c + 1) as usize] > 0 {
                                    if img_row11[(c + 2) as usize] > 0 {
                                        if img_row11[(c) as usize] > 0 {
                                            return Some(NODE_6);
                                        }
                                        else {
                                            return Some(NODE_14);
                                        }
                                    }
                                    else {
                                        if img_row11[(c) as usize] > 0 {
                                            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                            return Some(cl_tree_4);
                                        }
                                        else {
                                            img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                                            return Some(cl_tree_3);
                                        }
                                    }
                                }
                                else {
                                    if img_row11[(c) as usize] > 0 {
                                        img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                        return Some(cl_tree_10);
                                    }
                                    else {
                                        img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                                        return Some(cl_tree_9);
                                    }
                                }
                            }
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row00[(c - 1) as usize] > 0 {
                                return Some(NODE_16);
                            }
                            else {
                                if img_row01[(c - 1) as usize] > 0 {
                                    if img_row11[(c + 1) as usize] > 0 {
                                        return Some(NODE_17);
                                    }
                                    else {
                                        if img_row11[(c + 2) as usize] > 0 {
                                            return Some(NODE_19);
                                        }
                                        else {
                                            if img_row11[(c) as usize] > 0 {
                                                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                                return Some(cl_tree_4);
                                            }
                                            else {
                                                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                                return Some(cl_tree_3);
                                            }
                                        }
                                    }
                                }
                                else {
                                    return Some(NODE_2);
                                }
                            }
                        }
                        else {
                            if img_row01[(c - 1) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                return Some(cl_tree_7);
                            }
                            else {
                                if img_row00[(c - 1) as usize] > 0 {
                                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                    return Some(cl_tree_7);
                                }
                                else {
                                    img_labels_row00[c as usize] = solver.new_label();
                                    return Some(cl_tree_7);
                                }
                            }
                        }
                    }
                    else {
                        return Some(NODE_3);
                    }
                }
}
cl_tree_12 => {
if ({c+=2; c}) >= w - 2 { if c > w - 2 { return Some(cl_break_0_8); } else { return Some(cl_break_1_12); } }
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        if img_row11[(c + 1) as usize] > 0 {
                            if img_row11[(c - 2) as usize] > 0 {
                                return Some(NODE_34);
                            }
                            else {
                                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                return Some(cl_tree_11);
                            }
                        }
                        else {
                            if img_row00[(c + 1) as usize] > 0 {
                                return Some(NODE_35);
                            }
                            else {
                                if img_row11[(c) as usize] > 0 {
                                    if img_row11[(c - 2) as usize] > 0 {
                                        return Some(NODE_40);
                                    }
                                    else {
                                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                        return Some(cl_tree_10);
                                    }
                                }
                                else {
                                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                    return Some(cl_tree_9);
                                }
                            }
                        }
                    }
                    else {
                        return Some(NODE_15);
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row01[(c - 1) as usize] > 0 {
                                if img_row11[(c + 1) as usize] > 0 {
                                    if img_row11[(c - 2) as usize] > 0 {
                                        return Some(NODE_32);
                                    }
                                    else {
                                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                                        return Some(cl_tree_6);
                                    }
                                }
                                else {
                                    return Some(NODE_35);
                                }
                            }
                            else {
                                return Some(NODE_2);
                            }
                        }
                        else {
                            return Some(NODE_25);
                        }
                    }
                    else {
                        return Some(NODE_3);
                    }
                }
}
        NODE_41=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
        }
        else {
            if img_row11[(c - 1) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
            }
        }
                }
        NODE_42=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
                }
        NODE_43=> {
        if img_row01[(c) as usize] > 0 {
            img_labels_row00[c as usize] = solver.new_label();
        }
        else {
            img_labels_row00[c as usize] = 0;
        }
                }
        NODE_44=> {
        if img_row01[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
        else {
            img_labels_row00[c as usize] = 0;
        }
                }
        NODE_45=> {
        if img_row11[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c - 2) as usize], solver);
        }
        else {
        return Some(NODE_46);
        }
                }
        NODE_47=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
        }
        else {
            img_labels_row00[c as usize] = solver.new_label();
        }
                }
        NODE_48=> {
        if img_row01[(c) as usize] > 0 {
            if img_row01[(c - 1) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
            }
        }
        else {
            img_labels_row00[c as usize] = 0;
        }
                }
        NODE_46=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
cl_break_0_0 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_47);
                }
                else {
                    return Some(NODE_43);
                }
        return None;}
cl_break_0_1 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_41);
                }
                else {
                    return Some(NODE_43);
                }
        return None;}
cl_break_0_2 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_45);
                }
                else {
                    return Some(NODE_44);
                }
        return None;}
cl_break_0_3 => {
                if img_row00[(c) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                }
                else {
                    return Some(NODE_44);
                }
        return None;}
cl_break_0_4 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        return Some(NODE_45);
                    }
                    else {
                        return Some(NODE_41);
                    }
                }
                else {
                    return Some(NODE_48);
                }
        return None;}
cl_break_0_5 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        return Some(NODE_46);
                    }
                    else {
                        return Some(NODE_47);
                    }
                }
                else {
                    return Some(NODE_48);
                }
        return None;}
cl_break_0_6 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        if img_row11[(c) as usize] > 0 {
                            return Some(NODE_42);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_47);
                    }
                }
                else {
                    return Some(NODE_48);
                }
        return None;}
cl_break_0_7 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row00[(c - 1) as usize] > 0 {
                        img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                    }
                    else {
                        if img_row01[(c - 1) as usize] > 0 {
                            img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                        }
                        else {
                            if img_row11[(c) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                            }
                            else {
                                img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                            }
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row01[(c - 1) as usize] > 0 {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                        else {
                            if img_row00[(c - 1) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            }
                            else {
                                img_labels_row00[c as usize] = solver.new_label();
                            }
                        }
                    }
                    else {
                        img_labels_row00[c as usize] = 0;
                    }
                }
        return None;}
cl_break_0_8 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        if img_row11[(c) as usize] > 0 {
                            if img_row11[(c - 2) as usize] > 0 {
                                return Some(NODE_42);
                            }
                            else {
                                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                            }
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_47);
                    }
                }
                else {
                    return Some(NODE_48);
                }
        return None;}
        NODE_49=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row12[(c) as usize] > 0 {
            return Some(NODE_50);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            }
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
        NODE_51=> {
        if img_row01[(c - 1) as usize] > 0 {
            if img_row11[(c + 1) as usize] > 0 {
            return Some(NODE_52);
            }
            else {
                if img_row11[(c) as usize] > 0 {
                return Some(NODE_53);
                }
                else {
                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                }
            }
        }
        else {
        return Some(NODE_54);
        }
                }
        NODE_53=> {
        if img_row12[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
                }
        NODE_55=> {
        if img_row01[(c - 1) as usize] > 0 {
            if img_row11[(c + 1) as usize] > 0 {
                if img_row11[(c - 2) as usize] > 0 {
                return Some(NODE_52);
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                }
            }
            else {
                if img_row11[(c) as usize] > 0 {
                return Some(NODE_50);
                }
                else {
                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                }
            }
        }
        else {
        return Some(NODE_54);
        }
                }
        NODE_56=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row12[(c) as usize] > 0 {
            return Some(NODE_53);
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            }
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
        NODE_57=> {
        if img_row11[(c + 1) as usize] > 0 {
        return Some(NODE_58);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
        NODE_58=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
        }
        else {
            if img_row12[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            }
            else {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            }
        }
                }
        NODE_59=> {
        if img_row01[(c - 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
        else {
            img_labels_row00[c as usize] = solver.new_label();
        }
                }
        NODE_60=> {
        if img_row00[(c + 1) as usize] > 0 {
            if img_row11[(c + 1) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
            }
        }
        else {
        return Some(NODE_61);
        }
                }
        NODE_62=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
        NODE_52=> {
        if img_row12[(c - 1) as usize] > 0 {
        return Some(NODE_58);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
                }
        NODE_63=> {
        if img_row01[(c) as usize] > 0 {
            if img_row00[(c + 1) as usize] > 0 {
            return Some(NODE_64);
            }
            else {
            return Some(NODE_59);
            }
        }
        else {
        return Some(NODE_65);
        }
                }
        NODE_64=> {
        if img_row01[(c - 1) as usize] > 0 {
        return Some(NODE_66);
        }
        else {
        return Some(NODE_54);
        }
                }
        NODE_50=> {
        if img_row11[(c - 2) as usize] > 0 {
        return Some(NODE_53);
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
                }
        NODE_61=> {
        if img_row01[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = solver.new_label();
        }
        else {
            img_labels_row00[c as usize] = 0;
        }
                }
        NODE_67=> {
        if img_row11[(c) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
        else {
            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
        }
                }
        NODE_65=> {
        if img_row00[(c + 1) as usize] > 0 {
        return Some(NODE_54);
        }
        else {
        return Some(NODE_61);
        }
                }
        NODE_68=> {
        if img_row01[(c) as usize] > 0 {
            if img_row00[(c + 1) as usize] > 0 {
            return Some(NODE_54);
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
            }
        }
        else {
        return Some(NODE_65);
        }
                }
        NODE_66=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
        }
        else {
        return Some(NODE_67);
        }
                }
        NODE_69=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                    if img_row12[(c) as usize] > 0 {
                        img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                    }
                    else {
                        img_labels_row00[c as usize] = LabelsSolver::merge(LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver), img_labels_row12[(c - 2) as usize], solver);
                    }
                }
                else {
                    img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c) as usize], solver);
                }
            }
        }
        else {
            if img_row11[(c - 1) as usize] > 0 {
                img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row00[(c - 2) as usize], img_labels_row12[(c - 2) as usize], solver);
            }
            else {
            return Some(NODE_67);
            }
        }
                }
        NODE_70=> {
        if img_row12[(c) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
        }
        else {
            img_labels_row00[c as usize] = LabelsSolver::merge(img_labels_row12[(c) as usize], img_labels_row12[(c - 2) as usize], solver);
        }
                }
        NODE_54=> {
        if img_row11[(c + 1) as usize] > 0 {
            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
        }
        else {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            }
            else {
                img_labels_row00[c as usize] = solver.new_label();
            }
        }
                }
        NODE_71=> {
        if img_row11[(c + 1) as usize] > 0 {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                return Some(NODE_70);
                }
                else {
                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                }
            }
        }
        else {
            if img_row11[(c) as usize] > 0 {
                img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
            }
            else {
                if img_row11[(c - 1) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                }
                else {
                    img_labels_row00[c as usize] = solver.new_label();
                }
            }
        }
                }
cl_break_1_0 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_54);
                }
                else {
                    return Some(NODE_68);
                }
        return None;}
cl_break_1_1 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_71);
                }
                else {
                    return Some(NODE_68);
                }
        return None;}
cl_break_1_2 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_69);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_66);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_65);
                    }
                }
        return None;}
cl_break_1_3 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_62);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_62);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_60);
                    }
                }
        return None;}
cl_break_1_4 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_56);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_56);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_60);
                    }
                }
        return None;}
cl_break_1_5 => {
                if img_row00[(c) as usize] > 0 {
                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                    }
                    else {
                        if img_row00[(c + 1) as usize] > 0 {
                            img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                        }
                        else {
                            return Some(NODE_61);
                        }
                    }
                }
        return None;}
cl_break_1_6 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_57);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_57);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_65);
                    }
                }
        return None;}
cl_break_1_7 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row01[(c - 1) as usize] > 0 {
                        return Some(NODE_69);
                    }
                    else {
                        return Some(NODE_71);
                    }
                }
                else {
                    return Some(NODE_63);
                }
        return None;}
cl_break_1_8 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_49);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_49);
                        }
                        else {
                            img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                        }
                    }
                    else {
                        return Some(NODE_60);
                    }
                }
        return None;}
cl_break_1_9 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_64);
                }
                else {
                    return Some(NODE_63);
                }
        return None;}
cl_break_1_10 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_51);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_51);
                        }
                        else {
                            return Some(NODE_59);
                        }
                    }
                    else {
                        return Some(NODE_65);
                    }
                }
        return None;}
cl_break_1_11 => {
                if img_row00[(c) as usize] > 0 {
                    if img_row00[(c - 1) as usize] > 0 {
                        return Some(NODE_57);
                    }
                    else {
                        if img_row01[(c - 1) as usize] > 0 {
                            return Some(NODE_57);
                        }
                        else {
                            if img_row11[(c + 1) as usize] > 0 {
                                if img_row11[(c) as usize] > 0 {
                                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                }
                                else {
                                    return Some(NODE_70);
                                }
                            }
                            else {
                                if img_row11[(c) as usize] > 0 {
                                    img_labels_row00[c as usize] = img_labels_row12[(c) as usize];
                                }
                                else {
                                    img_labels_row00[c as usize] = img_labels_row12[(c - 2) as usize];
                                }
                            }
                        }
                    }
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            if img_row00[(c - 1) as usize] > 0 {
                                return Some(NODE_57);
                            }
                            else {
                                if img_row01[(c - 1) as usize] > 0 {
                                    return Some(NODE_57);
                                }
                                else {
                                    return Some(NODE_54);
                                }
                            }
                        }
                        else {
                            if img_row01[(c - 1) as usize] > 0 {
                                img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                            }
                            else {
                                if img_row00[(c - 1) as usize] > 0 {
                                    img_labels_row00[c as usize] = img_labels_row00[(c - 2) as usize];
                                }
                                else {
                                    img_labels_row00[c as usize] = solver.new_label();
                                }
                            }
                        }
                    }
                    else {
                        return Some(NODE_65);
                    }
                }
        return None;}
cl_break_1_12 => {
                if img_row00[(c) as usize] > 0 {
                    return Some(NODE_55);
                }
                else {
                    if img_row01[(c) as usize] > 0 {
                        if img_row00[(c + 1) as usize] > 0 {
                            return Some(NODE_55);
                        }
                        else {
                            return Some(NODE_59);
                        }
                    }
                    else {
                        return Some(NODE_65);
                    }
                }
        return None;}
    }; None})(label)
{
label = next;
}
}}
