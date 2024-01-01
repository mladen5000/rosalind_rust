using LinearAlgebra


function pca(X::Matrix{T}) where T<:AbstractFloat
    # Center the data
    X_centered = X .- mean(X, dims=1)
    
    # Compute the covariance matrix
    cov_matrix = cov(X_centered)
    
    # Compute the eigenvalues and eigenvectors of the covariance matrix
    eigenvals, eigenvecs = eigen(cov_matrix)
    
    # Sort the eigenvalues in descending order
    sorted_indices = sortperm(eigenvals, rev=true)
    eigenvals_sorted = eigenvals[sorted_indices]
    eigenvecs_sorted = eigenvecs[:, sorted_indices]
    
    # Compute the explained variance ratio
    explained_variance_ratio = eigenvals_sorted / sum(eigenvals_sorted)
    
    return eigenvals_sorted, eigenvecs_sorted, explained_variance_ratio
end
using Plots

# Call the pca function
eigenvals_sorted, eigenvecs_sorted, explained_variance_ratio = pca(X)

# Plot the explained variance ratio
plot(explained_variance_ratio, xlabel="Principal Components", ylabel="Explained Variance Ratio", title="PCA Explained Variance Ratio")
