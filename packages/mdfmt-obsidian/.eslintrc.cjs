module.exports = {
  env: {
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint'],
  settings: {
    'import/parsers': {
      '@typescript-eslint/parser': ['.ts', '.tsx'],
    },
    'import/resolver': {
      typescript: {
        project: [
          // For CLI
          'tsconfig.json',
          // For VSCode
          'packages/*/tsconfig.json',
        ],
      },
      node: true,
    },
  },
  rules: {
    'import/default': 'off',
    'import/no-unresolved': ['error'],
    'no-restricted-imports': [
      'error',
      {
        patterns: [
          '~/main',
          '../\\w*/*',
          '~/features/**/components/*',
          '../../**/components/*',
          '../**/__workaround__/*',
        ],
      },
    ],
    'import/order': [
      'error',
      {
        groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index'],
        alphabetize: {
          order: 'asc',
        },
        pathGroups: [
          {
            pattern: '~/**',
            group: 'internal',
          },
        ],
        'newlines-between': 'always',
      },
    ],
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
      },
    ],
    '@typescript-eslint/ban-types': ['off'],
  },
};
